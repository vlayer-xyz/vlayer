use crate::block_header::EvmBlockHeader;
use std::any::TypeId;

pub(crate) fn try_downcast<To: EvmBlockHeader + Clone>(
    header: &dyn EvmBlockHeader,
) -> Result<To, &'static str> {
    header
        .as_any()
        .downcast_ref::<To>()
        .cloned()
        .ok_or("Failed to downcast EvmBlockHeader")
}

pub(crate) fn is<T: EvmBlockHeader>(header: &dyn EvmBlockHeader) -> bool {
    header.as_any().type_id() == TypeId::of::<T>()
}
