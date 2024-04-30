/*
Plugin Name
Copyright (C) <Year> <Developer> <Email Address>

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program. If not, see <https://www.gnu.org/licenses/>
*/

#include "botwfd.h"
#include "setting-macros.h"

OBS_DECLARE_MODULE()
OBS_MODULE_USE_DEFAULT_LOCALE(PLUGIN_NAME, "en-US")

static const char* botwfdplugin_get_name(void *) {
    return "BotW Flight Director";
}

namespace botwfd {

bool is_format_supported(enum video_format format) {
    switch (format) {
        case VIDEO_FORMAT_YUY2:
            return true;
        default:
            return false;
    }
}

void filter_video(BotwFdCore *p_core, struct obs_source_frame *p_frame) {
    if (!is_format_supported(p_frame->format)) {
        return;
    }
    botwfd_process_frame(p_core);
}

}

/// Settings
/// See the core crate for what these values are
make_setting_(bool, enable)
make_setting_(bool, expose_host)
make_setting_cast_(int, port, uint16_t)

/// Get the default settings
static void botwfdplugin_get_defaults(obs_data_t *p_settings) {
    setting_set_default_(enable)(p_settings, true);
    setting_set_default_(expose_host)(p_settings, false);
    setting_set_default_(port)(p_settings, 8899);
}

/// Get the setting properties UI configuration
static obs_properties_t* botwfdplugin_get_properties(void *)
{
	obs_properties_t* props = obs_properties_create();

	//obs_properties_add_text(props, BOTW_QT_SETTING_MODULE_TEXT(FORMAT_NOTE), OBS_TEXT_INFO);
	obs_properties_add_bool(props, setting_(enable), localize_setting_(enable));
    obs_properties_add_bool(props, setting_(expose_host), localize_setting_(expose_host));
	obs_properties_add_int(props, setting_(port), localize_setting_(port), 2000, 65535, 1);
	// obs_property_t *p = obs_properties_add_int_slider(props, BOTW_QT_SETTING_MODULE_TEXT(RATE), 5, 60, 1);
	// obs_property_int_set_suffix(p, BOTW_QT_MODULE_TEXT(RATE_UNIT));
	// obs_properties_add_int_slider(props, BOTW_QT_SETTING_MODULE_TEXT(THRESHOLD), 0, 255, 1);

	return props;
}



static void botwfdplugin_update_settings(void *p_core, obs_data_t *p_settings) {
    // extract the setting values from OBS data
    auto* p_core_settings = botwfd_get_settings(reinterpret_cast<BotwFdCore*>(p_core));
    p_core_settings->enable = setting_get_(enable)(p_settings);
    p_core_settings->expose_host = setting_get_(expose_host)(p_settings);
    p_core_settings->port = setting_get_(port)(p_settings);

    // update the core according to the new settings
    botwfd_update_settings(reinterpret_cast<BotwFdCore*>(p_core));
}

static void* botwfdplugin_create(obs_data_t *p_settings, obs_source_t */*p_context*/) {
    obs_log(LOG_INFO, "creating instance");
    BotwFdCore* p_core = botwfd_create();
    botwfdplugin_update_settings(p_core, p_settings);

    return p_core;
}

static void botwfdplugin_destroy(void *p_core) {
    obs_log(LOG_INFO, "destroying instance");
    botwfd_destroy(reinterpret_cast<BotwFdCore*>(p_core));
    obs_log(LOG_INFO, "destroyed instance");
}

static struct obs_source_frame* botwfdplugin_filter_video(void *p_core, struct obs_source_frame *p_frame) {
    botwfd::filter_video(reinterpret_cast<BotwFdCore*>(p_core), p_frame);
    return p_frame;
}

struct obs_source_info botwfd_plugin;

bool obs_module_load(void) {
    obs_log(LOG_INFO, "loading plugin v%s", PLUGIN_VERSION);
    botwfd_plugin.id = "botwfd";
    botwfd_plugin.type = OBS_SOURCE_TYPE_FILTER;
    botwfd_plugin.output_flags = OBS_SOURCE_VIDEO | OBS_SOURCE_ASYNC;
    botwfd_plugin.get_name = botwfdplugin_get_name;
    botwfd_plugin.create = botwfdplugin_create;
    botwfd_plugin.destroy = botwfdplugin_destroy;
    botwfd_plugin.get_defaults = botwfdplugin_get_defaults,
    botwfd_plugin.get_properties = botwfdplugin_get_properties,
    botwfd_plugin.update = botwfdplugin_update_settings,
    botwfd_plugin.filter_video = botwfdplugin_filter_video;

    obs_register_source(&botwfd_plugin);
    obs_log(LOG_INFO, "source registered");
    botwfd_load();
    obs_log(LOG_INFO, "plugin loaded successfully)");
    return true;
}

void obs_module_unload(void) {
    obs_log(LOG_INFO, "plugin unloaded");
}
