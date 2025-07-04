use extraction_gym::*;
use extraction_gym::Extractor as extract_Extractor;
use indexmap::IndexMap;


#[derive(PartialEq, Eq)]
pub enum Optimal {
    Tree,
    DAG,
    Neither,
}

pub struct ExtractorDetail {
    extractor: Box<dyn extract_Extractor>,
    optimal: Optimal,
    use_for_bench: bool,
}

impl ExtractorDetail {
    // Getter for `extractor`
    pub fn get_extractor(&self) -> &dyn extract_Extractor {
        &*self.extractor
    }

    // Getter for `optimal`
    pub fn get_optimal(&self) -> &Optimal {
        &self.optimal
    }

    // Getter for `use_for_bench`
    pub fn get_use_for_bench(&self) -> bool {
        self.use_for_bench
    }
}

pub fn extractors() -> IndexMap<&'static str, ExtractorDetail> {
    let extractors: IndexMap<&'static str, ExtractorDetail> = [
        (
            "bottom-up",
            ExtractorDetail {
                extractor: extraction_gym::bottom_up::BottomUpExtractor.boxed(),
                optimal: Optimal::Tree,
                use_for_bench: true,
            },
        ),
        (
            "faster-bottom-up",
            ExtractorDetail {
                extractor: extraction_gym::faster_bottom_up::FasterBottomUpExtractor.boxed(),
                optimal: Optimal::Tree,
                use_for_bench: true,
            },
        ),
        (
            "faster-greedy-dag",
            ExtractorDetail {
                extractor: extraction_gym::faster_greedy_dag::FasterGreedyDagExtractor.boxed(),
                optimal: Optimal::Neither,
                use_for_bench: true,
            },
        ),
        (
            "faster-greedy-dag_fa",
            ExtractorDetail {
                extractor: extraction_gym::faster_greedy_dag_fa::FasterGreedyDagExtractor.boxed(),
                optimal: Optimal::Neither,
                use_for_bench: true,
            },
        ),
        (
            "faster-greedy-dag_fa_mt",
            ExtractorDetail {
                extractor: extraction_gym::faster_greedy_dag_fa_mt::FasterGreedyDagExtractor.boxed(),
                optimal: Optimal::Neither,
                use_for_bench: true,
            },
        ),
        (
            "global-greedy-dag",
            ExtractorDetail {
                extractor: extraction_gym::global_greedy_dag::GlobalGreedyDagExtractor.boxed(),
                optimal: Optimal::Neither,
                use_for_bench: true,
            },
        ),
        #[cfg(feature = "ilp-cbc")]
        (
            "ilp-cbc-timeout",
            ExtractorDetail {
                extractor: extraction_gym::ilp_cbc::CbcExtractorWithTimeout::<10>.boxed(),
                optimal: Optimal::DAG,
                use_for_bench: true,
            },
        ),
        #[cfg(feature = "ilp-cbc")]
        (
            "ilp-cbc",
            ExtractorDetail {
                extractor: extraction_gym::ilp_cbc::CbcExtractor.boxed(),
                optimal: Optimal::DAG,
                use_for_bench: false, // takes >10 hours sometimes
            },
        ),
        #[cfg(feature = "ilp-cbc")]
        (
            "faster-ilp-cbc-timeout",
            ExtractorDetail {
                extractor: extraction_gym::faster_ilp_cbc::FasterCbcExtractorWithTimeout::<10>.boxed(),
                optimal: Optimal::DAG,
                use_for_bench: true,
            },
        ),
        #[cfg(feature = "ilp-cbc")]
        (
            "faster-ilp-cbc",
            ExtractorDetail {
                extractor: extraction_gym::faster_ilp_cbc::FasterCbcExtractor.boxed(),
                optimal: Optimal::DAG,
                use_for_bench: true,
            },
        ),
        #[cfg(feature = "ilp-cbc")]
        (
            "ilp-cbc",
            ExtractorDetail {
                extractor: extraction_gym::faster_ilp_cbc::FasterCbcExtractor.boxed(),
                optimal: Optimal::DAG,
                use_for_bench: true,
            },
        ),
    ]
    .into_iter()
    .collect();
    return extractors;
}
