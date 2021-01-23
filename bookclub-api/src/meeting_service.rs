use crate::meeting_repository::MeetingRepository;

/// Represents the meetings domain.
pub struct MeetingService {
    repository: MeetingRepository,
}

impl MeetingService {
    /// Creates a new service.
    pub fn new(repository: MeetingRepository) -> Self {
        Self { repository }
    }
}
