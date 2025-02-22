// Test that warnings about unknown "#[testonly]" attribute is
// suppressed in libra2_std module.
module libra2_std::module_with_suppressed_warnings {
    #[a, a(x = 0)]
    fun foo() {}

    #[testonly]
    #[b(a, a = 0, a(x = 1))]
    fun bar() {}
}
