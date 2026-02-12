# MusicBrainz GraphQL API

A GraphQL API for the MusicBrainz database built with rust, featuring redis caching & dataloader batching.
Supports artist & release entities right now.

```
pub struct Artist{
    pub id: String,
    pub name: String
}

pub struct  Release{
    pub gid: String,
    pub name: String
}
```

## Quick Start

### Prerequisites

- rust 1.70
- musicbrainz-docker
- redis

### Installation

```bash
cargo build --release
cargo run
```
### Configuration

**Environment Variables:**
- `USE_DATALOADER`: Enable/disable DataLoader batching (default: `true`)

## Query Examples

It supports 3 primary types of queries:
- Search
- Lookup
- Browse

### Search
Full-text search with pagination:

```graphql
query {
  searchArtist(name: "Beatles", limit: 10, offset: 0) {
    id
    name
  }
}
```
case insensitive partial matching using `ILIKE`.

### Lookup

Retreive a single artist or release by ID:

```graphql
# Artist Lookup
query {
  artist(id: "5b11f4ce-a62d-471e-81fc-a69a8278c7da") {
    id
    name
  }
}

# Release Lookup with Artist Information
query {
  release(id: "f5093c06-23e3-404f-aeaa-40f72885ee3a") {
    gid
    name
    artist {
      id
      name
    }
  }
}
```

Retreive multiple artists as well:
```graphql
# Fetch multiple artists at once
query {
  artists(ids: [
    "5b11f4ce-a62d-471e-81fc-a69a8278c7da",
    "83d91898-7763-47d7-b03b-b92132375c47",
    "f82bcf78-5b69-4622-a5ef-73800768d9ac"
  ]) {
    id
    name
  }
}
```

**Limit**: Maximum 1000 IDs per request to prevent abuse.

### Browse

```graphql
# Browse an artist's releases
query {
  artist(id: "5b11f4ce-a62d-471e-81fc-a69a8278c7da") {
    id
    name
    release(limit: 10, offset: 0) {
      gid
      name
    }
  }
}

# Browse with pagination
query {
  artist(id: "5b11f4ce-a62d-471e-81fc-a69a8278c7da") {
    name
    release(limit: 20, offset: 20) {
      gid
      name
    }
  }
}
```

**Pagination**: Default limit is 20, maximum is 100. Use `offset` for pagination.

## How performance is made a prioity

We use Dataloaders to avoid the N+1 problem in GraphQL. Here we have implemented it for browsing Artist using Releases. 
The performance shift can be seen using the performance output by comapring the process run with and without the `USE_DATALOADER` as set.

We've also included a limit_depth of 5 and max_complexity of 200. These can be configured in the main.rs file and individual complexities in the models.rs file.

for example -> 
query
```
{
  artist(id: "5b11f4ce-a62d-471e-81fc-a69a8278c7da") {
    name
    release(limit: 10) {
      name
      artist {
        name
      }
    }
  }
}
```
With Dataloader:
```
┌────────────────────────────────────────┐
│  📊 GraphQL Request Metrics            │
├────────────────────────────────────────┤
│  Total DB Queries:    2               │
│  Request Duration:   20ms             │
│  Avg per query:      10ms             │
│  DataLoader:       ✅ ENABLED          │
└────────────────────────────────────────┘
```
Without Dataloader:
```
┌────────────────────────────────────────┐
│  📊 GraphQL Request Metrics            │
├────────────────────────────────────────┤
│  Total DB Queries:   10               │
│  Request Duration:  713ms             │
│  Avg per query:      71ms             │
│  DataLoader:       ❌ DISABLED         │
└────────────────────────────────────────┘

```
## Overview
<img width="1214" height="767" alt="image" src="https://github.com/user-attachments/assets/5e5a3d94-cbd7-4c82-957d-863f0d51aab7" />


## Things to include further

- Batch Caching
- Negative Caches
- Better Error Handling
