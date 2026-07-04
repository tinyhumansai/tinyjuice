#[cfg(test)]
mod tests {
    use crate::openhuman::OpenHumanCompressionContext;

    #[test]
    fn openhuman_context_starts_empty() {
        let context = OpenHumanCompressionContext::default();

        assert_eq!(context.conversation_id, None);
        assert_eq!(context.model, None);
    }
}
