---
"muda": "minor"
---

**Breaking Change** Removed `AboutMetadata::from_cargo_metadata` and `AboutMetadataBuilder::with_cargo_metadata` which had incorrect implementation, use the new `about_metadata::from_cargo_metadata` macro instead.
