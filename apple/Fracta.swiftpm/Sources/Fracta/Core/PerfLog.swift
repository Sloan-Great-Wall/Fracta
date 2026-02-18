import Foundation
import os

/// Lightweight performance logging for validating Milestone 0.1 targets:
/// - Cold start: < 2 seconds (app launch → first content visible)
/// - View switch: < 0.3 seconds (file tap → document rendered)
///
/// Uses `os.signpost` intervals for Instruments integration and prints
/// human-readable timing to the console during development.
///
/// All logging is compiled out in release builds via the DEBUG flag.
enum PerfLog {
    static let logger = Logger(subsystem: "com.fracta.app", category: "Performance")
    static let signposter = OSSignposter(logger: logger)

    /// Measure a synchronous block and log the result.
    @discardableResult
    static func measure<T>(_ label: String, block: () throws -> T) rethrows -> T {
        #if DEBUG
        let id = signposter.makeSignpostID()
        let state = signposter.beginInterval("Measure", id: id)
        let start = CFAbsoluteTimeGetCurrent()
        defer {
            let elapsed = CFAbsoluteTimeGetCurrent() - start
            signposter.endInterval("Measure", state)
            let ms = elapsed * 1000
            logger.info("[Perf] \(label): \(String(format: "%.1f", ms)) ms")
        }
        return try block()
        #else
        return try block()
        #endif
    }

    /// Begin a timed span. Call `end()` on the returned token to log.
    static func begin(_ label: String) -> PerfSpan {
        #if DEBUG
        let id = signposter.makeSignpostID()
        let state = signposter.beginInterval("Span", id: id)
        return PerfSpan(label: label, start: CFAbsoluteTimeGetCurrent(), state: state)
        #else
        return PerfSpan(label: label, start: 0, state: nil)
        #endif
    }

    /// Log a one-shot timing value (for when you already have the duration).
    static func log(_ label: String, ms: Double) {
        #if DEBUG
        logger.info("[Perf] \(label): \(String(format: "%.1f", ms)) ms")
        #endif
    }

    /// Validate a timing against a target and warn if exceeded.
    static func validate(_ label: String, ms: Double, target: Double) {
        #if DEBUG
        if ms > target {
            logger.warning("[Perf] ⚠️ \(label): \(String(format: "%.1f", ms)) ms EXCEEDS target \(String(format: "%.0f", target)) ms")
        } else {
            logger.info("[Perf] ✓ \(label): \(String(format: "%.1f", ms)) ms (target: \(String(format: "%.0f", target)) ms)")
        }
        #endif
    }
}

/// A timed span token. Call `end()` to stop timing and log.
struct PerfSpan {
    let label: String
    let start: CFAbsoluteTime
    let state: OSSignpostIntervalState?

    /// End the span and log the elapsed time.
    @discardableResult
    func end() -> Double {
        #if DEBUG
        let elapsed = (CFAbsoluteTimeGetCurrent() - start) * 1000
        if let state {
            PerfLog.signposter.endInterval("Span", state)
        }
        PerfLog.log(label, ms: elapsed)
        return elapsed
        #else
        return 0
        #endif
    }

    /// End the span and validate against a target.
    @discardableResult
    func validate(target: Double) -> Double {
        #if DEBUG
        let elapsed = (CFAbsoluteTimeGetCurrent() - start) * 1000
        if let state {
            PerfLog.signposter.endInterval("Span", state)
        }
        PerfLog.validate(label, ms: elapsed, target: target)
        return elapsed
        #else
        return 0
        #endif
    }
}
