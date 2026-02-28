//! `.acon` binary file format — portable contract store.

use std::io::{Read, Write};
use std::path::PathBuf;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::approval::{ApprovalDecision, ApprovalRequest, ApprovalRule};
use crate::condition::Condition;
use crate::error::{ContractError, ContractResult};
use crate::obligation::Obligation;
use crate::policy::Policy;
use crate::risk_limit::RiskLimit;
use crate::violation::Violation;
use crate::ContractId;

/// Magic bytes identifying `.acon` files.
pub const MAGIC: [u8; 4] = *b"ACON";

/// Current format version.
pub const VERSION: u32 = 1;

/// File header (fixed size).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct FileHeader {
    /// Magic bytes "ACON".
    pub magic: [u8; 4],
    /// Format version.
    pub version: u32,
    /// Flags (reserved).
    pub flags: u32,
    /// Number of policies.
    pub policy_count: u64,
    /// Number of risk limits.
    pub risk_limit_count: u64,
    /// Number of approval rules.
    pub approval_rule_count: u64,
    /// Number of approval requests.
    pub approval_request_count: u64,
    /// Number of conditions.
    pub condition_count: u64,
    /// Number of obligations.
    pub obligation_count: u64,
    /// Number of violations.
    pub violation_count: u64,
    /// File creation timestamp (Unix micros).
    pub created_at: u64,
    /// Last modified timestamp (Unix micros).
    pub modified_at: u64,
    /// BLAKE3 checksum.
    pub checksum: [u8; 32],
}

impl Default for FileHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl FileHeader {
    /// Create a new header with current timestamps.
    pub fn new() -> Self {
        let now = Utc::now().timestamp_micros() as u64;
        Self {
            magic: MAGIC,
            version: VERSION,
            flags: 0,
            policy_count: 0,
            risk_limit_count: 0,
            approval_rule_count: 0,
            approval_request_count: 0,
            condition_count: 0,
            obligation_count: 0,
            violation_count: 0,
            created_at: now,
            modified_at: now,
            checksum: [0; 32],
        }
    }

    /// Write header to a writer.
    pub fn write_to<W: Write>(&self, writer: &mut W) -> ContractResult<()> {
        writer.write_all(&self.magic)?;
        writer.write_all(&self.version.to_le_bytes())?;
        writer.write_all(&self.flags.to_le_bytes())?;
        writer.write_all(&self.policy_count.to_le_bytes())?;
        writer.write_all(&self.risk_limit_count.to_le_bytes())?;
        writer.write_all(&self.approval_rule_count.to_le_bytes())?;
        writer.write_all(&self.approval_request_count.to_le_bytes())?;
        writer.write_all(&self.condition_count.to_le_bytes())?;
        writer.write_all(&self.obligation_count.to_le_bytes())?;
        writer.write_all(&self.violation_count.to_le_bytes())?;
        writer.write_all(&self.created_at.to_le_bytes())?;
        writer.write_all(&self.modified_at.to_le_bytes())?;
        writer.write_all(&self.checksum)?;
        Ok(())
    }

    /// Read header from a reader.
    pub fn read_from<R: Read>(reader: &mut R) -> ContractResult<Self> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;

        if magic != MAGIC {
            return Err(ContractError::FileFormat(format!(
                "Invalid magic bytes: {:?} (expected {:?}). File may be corrupted.",
                magic, MAGIC
            )));
        }

        let mut buf4 = [0u8; 4];
        let mut buf8 = [0u8; 8];
        let mut checksum = [0u8; 32];

        reader.read_exact(&mut buf4)?;
        let version = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf4)?;
        let flags = u32::from_le_bytes(buf4);

        reader.read_exact(&mut buf8)?;
        let policy_count = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let risk_limit_count = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let approval_rule_count = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let approval_request_count = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let condition_count = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let obligation_count = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let violation_count = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let created_at = u64::from_le_bytes(buf8);

        reader.read_exact(&mut buf8)?;
        let modified_at = u64::from_le_bytes(buf8);

        reader.read_exact(&mut checksum)?;

        Ok(Self {
            magic,
            version,
            flags,
            policy_count,
            risk_limit_count,
            approval_rule_count,
            approval_request_count,
            condition_count,
            obligation_count,
            violation_count,
            created_at,
            modified_at,
            checksum,
        })
    }
}

/// Entity types stored in file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum EntityType {
    /// Policy rule.
    Policy = 1,
    /// Risk limit.
    RiskLimit = 2,
    /// Approval rule.
    ApprovalRule = 3,
    /// Approval request.
    ApprovalRequest = 4,
    /// Approval decision.
    ApprovalDecision = 5,
    /// Condition.
    Condition = 6,
    /// Obligation.
    Obligation = 7,
    /// Violation.
    Violation = 8,
}

impl TryFrom<u8> for EntityType {
    type Error = ContractError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(EntityType::Policy),
            2 => Ok(EntityType::RiskLimit),
            3 => Ok(EntityType::ApprovalRule),
            4 => Ok(EntityType::ApprovalRequest),
            5 => Ok(EntityType::ApprovalDecision),
            6 => Ok(EntityType::Condition),
            7 => Ok(EntityType::Obligation),
            8 => Ok(EntityType::Violation),
            _ => Err(ContractError::FileFormat(format!(
                "Unknown entity type: {}",
                value
            ))),
        }
    }
}

/// In-memory representation of a `.acon` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractFile {
    /// Policies.
    #[serde(default)]
    pub policies: Vec<Policy>,
    /// Risk limits.
    #[serde(default)]
    pub risk_limits: Vec<RiskLimit>,
    /// Approval rules.
    #[serde(default)]
    pub approval_rules: Vec<ApprovalRule>,
    /// Approval requests.
    #[serde(default)]
    pub approval_requests: Vec<ApprovalRequest>,
    /// Approval decisions.
    #[serde(default)]
    pub approval_decisions: Vec<ApprovalDecision>,
    /// Conditions.
    #[serde(default)]
    pub conditions: Vec<Condition>,
    /// Obligations.
    #[serde(default)]
    pub obligations: Vec<Obligation>,
    /// Violations.
    #[serde(default)]
    pub violations: Vec<Violation>,
    /// Source file path (not serialized in the file body).
    #[serde(skip)]
    pub path: Option<PathBuf>,
}

impl Default for ContractFile {
    fn default() -> Self {
        Self::new()
    }
}

impl ContractFile {
    /// Create a new empty contract file.
    pub fn new() -> Self {
        Self {
            policies: vec![],
            risk_limits: vec![],
            approval_rules: vec![],
            approval_requests: vec![],
            approval_decisions: vec![],
            conditions: vec![],
            obligations: vec![],
            violations: vec![],
            path: None,
        }
    }

    /// Create from a file path, loading if it exists.
    pub fn open(path: impl Into<PathBuf>) -> ContractResult<Self> {
        let path = path.into();
        if path.exists() {
            Self::load(&path)
        } else {
            let mut file = Self::new();
            file.path = Some(path);
            Ok(file)
        }
    }

    /// Load from file.
    pub fn load(path: &std::path::Path) -> ContractResult<Self> {
        let mut reader = std::io::BufReader::new(std::fs::File::open(path)?);
        let _header = FileHeader::read_from(&mut reader)?;

        // Read JSON body after header
        let mut body = String::new();
        reader.read_to_string(&mut body)?;

        let mut file: ContractFile = serde_json::from_str(&body)?;
        file.path = Some(path.to_path_buf());
        Ok(file)
    }

    /// Save to file.
    pub fn save(&self) -> ContractResult<()> {
        let path = self
            .path
            .as_ref()
            .ok_or_else(|| ContractError::FileFormat("No file path set".to_string()))?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut writer = std::io::BufWriter::new(std::fs::File::create(path)?);

        let mut header = FileHeader::new();
        header.policy_count = self.policies.len() as u64;
        header.risk_limit_count = self.risk_limits.len() as u64;
        header.approval_rule_count = self.approval_rules.len() as u64;
        header.approval_request_count = self.approval_requests.len() as u64;
        header.condition_count = self.conditions.len() as u64;
        header.obligation_count = self.obligations.len() as u64;
        header.violation_count = self.violations.len() as u64;

        // Serialize body first to compute checksum
        let body = serde_json::to_string(self)?;
        header.checksum = *blake3::hash(body.as_bytes()).as_bytes();

        header.write_to(&mut writer)?;
        writer.write_all(body.as_bytes())?;

        Ok(())
    }

    /// Get total entity count across all types.
    pub fn total_entities(&self) -> usize {
        self.policies.len()
            + self.risk_limits.len()
            + self.approval_rules.len()
            + self.approval_requests.len()
            + self.approval_decisions.len()
            + self.conditions.len()
            + self.obligations.len()
            + self.violations.len()
    }

    /// Find a policy by ID.
    pub fn find_policy(&self, id: ContractId) -> Option<&Policy> {
        self.policies.iter().find(|p| p.id == id)
    }

    /// Find a risk limit by ID.
    pub fn find_risk_limit(&self, id: ContractId) -> Option<&RiskLimit> {
        self.risk_limits.iter().find(|r| r.id == id)
    }

    /// Find a mutable risk limit by ID.
    pub fn find_risk_limit_mut(&mut self, id: ContractId) -> Option<&mut RiskLimit> {
        self.risk_limits.iter_mut().find(|r| r.id == id)
    }

    /// Find an obligation by ID.
    pub fn find_obligation(&self, id: ContractId) -> Option<&Obligation> {
        self.obligations.iter().find(|o| o.id == id)
    }

    /// Find a mutable obligation by ID.
    pub fn find_obligation_mut(&mut self, id: ContractId) -> Option<&mut Obligation> {
        self.obligations.iter_mut().find(|o| o.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_roundtrip() {
        let header = FileHeader::new();
        let mut buf = Vec::new();
        header.write_to(&mut buf).unwrap();

        let parsed = FileHeader::read_from(&mut buf.as_slice()).unwrap();
        assert_eq!(parsed.magic, MAGIC);
        assert_eq!(parsed.version, VERSION);
    }

    #[test]
    fn test_contract_file_new() {
        let file = ContractFile::new();
        assert_eq!(file.total_entities(), 0);
        assert!(file.policies.is_empty());
    }

    #[test]
    fn test_bad_magic() {
        let bad = b"BADMxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
        let result = FileHeader::read_from(&mut bad.as_slice());
        assert!(result.is_err());
    }
}
