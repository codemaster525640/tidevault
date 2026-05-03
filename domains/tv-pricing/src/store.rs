use crate::model::{PricingFilter, PricingRecord};
use tv_kernel::error::{KernelError, Result};
use tv_kernel::ids::EntityId;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct PricingStore {
    inner: HashMap<EntityId, PricingRecord>,
    by_ref: HashMap<String, EntityId>,
}

impl PricingStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, rec: PricingRecord) -> Result<()> {
        if self.by_ref.contains_key(&rec.reference) {
            return Err(KernelError::Conflict(format!(
                "pricing: reference {} already exists",
                rec.reference
            )));
        }
        self.by_ref.insert(rec.reference.clone(), rec.id);
        self.inner.insert(rec.id, rec);
        Ok(())
    }

    pub fn get(&self, id: EntityId) -> Result<&PricingRecord> {
        self.inner
            .get(&id)
            .ok_or_else(|| KernelError::NotFound(format!("{id}")))
    }

    pub fn get_mut(&mut self, id: EntityId) -> Result<&mut PricingRecord> {
        self.inner
            .get_mut(&id)
            .ok_or_else(|| KernelError::NotFound(format!("{id}")))
    }

    pub fn get_by_ref(&self, reference: &str) -> Result<&PricingRecord> {
        let id = self
            .by_ref
            .get(reference)
            .copied()
            .ok_or_else(|| KernelError::NotFound(reference.into()))?;
        self.get(id)
    }

    pub fn update(&mut self, rec: PricingRecord) -> Result<()> {
        if !self.inner.contains_key(&rec.id) {
            return Err(KernelError::NotFound(format!("{}", rec.id)));
        }
        self.inner.insert(rec.id, rec);
        Ok(())
    }

    pub fn remove(&mut self, id: EntityId) -> Result<PricingRecord> {
        let rec = self
            .inner
            .remove(&id)
            .ok_or_else(|| KernelError::NotFound(format!("{id}")))?;
        self.by_ref.remove(&rec.reference);
        Ok(rec)
    }

    pub fn query(&self, filter: &PricingFilter) -> Vec<&PricingRecord> {
        let mut items: Vec<_> = self.inner.values().filter(|r| filter.matches(r)).collect();
        items.sort_by_key(|r| std::cmp::Reverse(r.created_at));
        items
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn checksum_all(&self) -> String {
        let mut ids: Vec<_> = self.inner.keys().copied().collect();
        ids.sort_by_key(|i| i.as_uuid());
        let payload = ids
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join("|");
        tv_kernel::hashutil::sha256_hex(format!("{payload}|61").as_bytes())
    }
}
