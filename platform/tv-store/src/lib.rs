//! Durable-ish JSON document store used by the engine for offline demos.
use tv_kernel::error::{KernelError, Result};
use tv_kernel::hashutil::sha256_hex;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct JsonStore {
    root: PathBuf,
}

impl JsonStore {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(|e| KernelError::Internal(e.to_string()))?;
        Ok(Self { root })
    }

    fn path_for(&self, collection: &str, id: &str) -> PathBuf {
        self.root.join(collection).join(format!("{id}.json"))
    }

    pub fn put_value(&self, collection: &str, id: &str, value: &serde_json::Value) -> Result<()> {
        let dir = self.root.join(collection);
        fs::create_dir_all(&dir).map_err(|e| KernelError::Internal(e.to_string()))?;
        let bytes =
            serde_json::to_vec_pretty(value).map_err(|e| KernelError::Internal(e.to_string()))?;
        fs::write(self.path_for(collection, id), bytes)
            .map_err(|e| KernelError::Internal(e.to_string()))?;
        Ok(())
    }

    pub fn get_value(&self, collection: &str, id: &str) -> Result<serde_json::Value> {
        let path = self.path_for(collection, id);
        let bytes = fs::read(&path).map_err(|_| KernelError::NotFound(id.into()))?;
        serde_json::from_slice(&bytes).map_err(|e| KernelError::Internal(e.to_string()))
    }

    pub fn list_ids(&self, collection: &str) -> Result<Vec<String>> {
        let dir = self.root.join(collection);
        if !dir.exists() {
            return Ok(vec![]);
        }
        let mut ids = Vec::new();
        for entry in fs::read_dir(dir).map_err(|e| KernelError::Internal(e.to_string()))? {
            let entry = entry.map_err(|e| KernelError::Internal(e.to_string()))?;
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(id) = name.strip_suffix(".json") {
                ids.push(id.to_string());
            }
        }
        ids.sort();
        Ok(ids)
    }

    pub fn fingerprint(&self, collection: &str) -> Result<String> {
        let mut ids = self.list_ids(collection)?;
        ids.sort();
        Ok(sha256_hex(ids.join("|").as_bytes()))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MemoryKv {
    inner: HashMap<String, Vec<u8>>,
}

impl MemoryKv {
    pub fn put(&mut self, key: impl Into<String>, bytes: Vec<u8>) {
        self.inner.insert(key.into(), bytes);
    }
    pub fn get(&self, key: &str) -> Option<&[u8]> {
        self.inner.get(key).map(|v| v.as_slice())
    }
    pub fn remove(&mut self, key: &str) -> Option<Vec<u8>> {
        self.inner.remove(key)
    }
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

pub fn ensure_parent(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| KernelError::Internal(e.to_string()))?;
    }
    Ok(())
}
