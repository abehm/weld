/// TODO
///
use super::WeldConf;
use super::error::WeldResult;
use super::passes::OPTIMIZATION_PASSES;
use super::passes::Pass;

use std::path::PathBuf;

#[derive(Clone,Deserialize,Serialize)]
pub struct ParsedConf {
    ///
    ///
    ///
    #[serde(rename="weld.memory.limit")]
    #[serde(default="ParsedConf::default_memory_limit")]
    pub memory_limit: i64,
    ///
    ///
    ///
    #[serde(rename="weld.threads")]
    #[serde(default="ParsedConf::default_threads")]
    pub threads: i32,
    ///
    ///
    ///
    #[serde(rename="weld.compile.multithreadSupport")]
    #[serde(default="ParsedConf::default_support_multithread")]
    ///
    ///
    ///
    pub support_multithread: bool,
    #[serde(rename="weld.compile.traceExecution")]
    #[serde(default="ParsedConf::default_trace_run")]
    ///
    ///
    ///
    pub trace_run: bool,
    #[serde(rename="weld.optimization.sirOptimization")]
    #[serde(default="ParsedConf::default_enable_sir_opt")]
    ///
    ///
    ///
    pub enable_sir_opt: bool,
    #[serde(rename="weld.optimization.applyExperimentalTransforms")]
    #[serde(default="ParsedConf::default_enable_experimental_passes")]
    ///
    ///
    ///
    pub enable_experimental_passes: bool,
    #[serde(rename="weld.optimization.passes")]
    #[serde(default="ParsedConf::default_optimization_passes")]
    ///
    ///
    ///
    pub optimization_passes: Vec<Pass>,
    #[serde(rename="weld.llvm.optimization.level")]
    #[serde(default="ParsedConf::default_llvm_optimization_level")]
    ///
    ///
    ///
    pub llvm_optimization_level: u32,
    #[serde(rename="weld.compile.dumpCode")]
    #[serde(default="ParsedConf::default_dump_code_enabled")]
    ///
    ///
    ///
    pub dump_code_enabled: bool,
    #[serde(rename="weld.compile.dumpCodeDir")]
    #[serde(default="ParsedConf::default_dump_code_dir")]
    ///
    ///
    ///
    pub dump_code_dir: PathBuf,
}

// Default values for initializers.

impl ParsedConf {
    fn default_memory_limit() -> i64 {
        1000000000
    }

    fn default_threads() -> i32 {
        1
    }

    fn default_support_multithread() -> bool {
        true
    }

    fn default_trace_run() -> bool {
        false 
    }

    fn default_enable_sir_opt() -> bool { 
        true
    }

    fn default_enable_experimental_passes() -> bool {
        false
    }

    fn default_optimization_passes() -> Vec<Pass> {
        let passes = ["loop-fusion", "unroll-static-loop", "infer-size", "short-circuit-booleans",
        "predicate", "vectorize", "fix-iterate"];
        passes.iter().map(|e| (*OPTIMIZATION_PASSES.get(e).unwrap()).clone()).collect()
    }

    fn default_llvm_optimization_level() -> u32 {
        2
    }

    fn default_dump_code_enabled() -> bool {
        false
    }

    fn default_dump_code_dir() -> PathBuf {
        PathBuf::from(".")
    }
}


impl Default for ParsedConf {
    fn default() -> ParsedConf {
        ParsedConf {
            memory_limit: ParsedConf::default_memory_limit(),
            threads: ParsedConf::default_threads(),
            support_multithread: ParsedConf::default_support_multithread(),
            trace_run: ParsedConf::default_trace_run(),
            enable_sir_opt: ParsedConf::default_enable_sir_opt(),
            enable_experimental_passes: ParsedConf::default_enable_experimental_passes(),
            optimization_passes: ParsedConf::default_optimization_passes(),
            llvm_optimization_level: ParsedConf::default_llvm_optimization_level(),
            dump_code_enabled: ParsedConf::default_dump_code_enabled(),
            dump_code_dir: ParsedConf::default_dump_code_dir(),
        }
    }
}
