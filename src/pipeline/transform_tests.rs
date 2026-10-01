use super::*;

#[test]
fn offload_output_requires_retained_ccr_put() {
    let token = "a1b2c3d4".repeat(4);
    let rejected = CcrPutResult::new(token.clone(), false);
    assert!(
        OffloadOutput::from_retained_put("partial".to_string(), CompressorKind::Generic, rejected,)
            .is_none()
    );

    let retained = CcrPutResult::new(token.clone(), true);
    let out =
        OffloadOutput::from_retained_put("partial".to_string(), CompressorKind::Generic, retained)
            .expect("retained CCR put constructs offload output");
    assert_eq!(out.text(), "partial");
    assert_eq!(out.token(), token);
}
