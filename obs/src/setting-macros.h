#pragma once

#ifdef _MSC_VER
#define please_inline_ inline __forceinline
#else
#define please_inline_ inline __attribute__((always_inline))
#endif

/// Define a new setting like 
///     make_setting_(bool, enable_foo)
///     make_setting_cast_(int, enable_foo, uint32_t)

#define make_setting_cast_(type, name, cast_type) \
    static please_inline_ const char* __botwfd_setting_##name() \
    { return #name; } \
    static please_inline_ const char* __botwfd_settinglong_##name() \
    { return "botwfd_setting_"#name; } \
    static please_inline_ void __botwfd_setting_set_default_##name(obs_data_t *p_settings, type value) \
    { obs_data_set_default_##type(p_settings, #name, value); } \
    static please_inline_ cast_type __botwfd_setting_get_##name(obs_data_t *p_settings) \
    { return static_cast<cast_type>(obs_data_get_##type(p_settings, #name)); }

#define make_setting_(type, name) make_setting_cast_(type, name, type)
/// Access the string setting key like
///     setting_(enable_foo)
#define setting_(name) __botwfd_setting_##name()

/// Set the default value for a setting like
///     setting_set_default_(enable_foo)(p_setting, true)
#define setting_set_default_(name) __botwfd_setting_set_default_##name

/// Get the value of a setting like
///     setting_get_(enable_foo)(p_settings)
#define setting_get_(name) __botwfd_setting_get_##name

/// Define a new localized message that's not a setting value like
///     #define make_message_(foo)
#define make_message_(name) \
    static please_inline_ const char* __botwfd_message_##name() { return "botwfd_message_"#name; }

/// Access the localized setting message like
///     localize_setting_(enable_foo)
#define localize_setting_(name) obs_module_text(__botwfd_settinglong_##name())

/// Access the localized message like
///     localize_message_(foo)
#define localize_message_(name) obs_module_text(__botwfd_message_##name())