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

OBS_DECLARE_MODULE()
OBS_MODULE_USE_DEFAULT_LOCALE(PLUGIN_NAME, "en-US")

// extern "C" {
//     /// Logging interface for the core (Rust side)
//     void botwfdplugin_log(const char* p_message) {
//         obs_log(LOG_INFO, "[core] %s", p_message);
//     }
// }

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
}

}

static void botwfdplugin_update_settings(void *p_core, obs_data_t *p_settings) {
    // extract the setting values from OBS data
    /* TODO */

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

extern "C" bool obs_module_load(void) {
    obs_log(LOG_INFO, "loading plugin v%s", PLUGIN_VERSION);
    botwfd_plugin.id = "botwfd";
    botwfd_plugin.type = OBS_SOURCE_TYPE_FILTER;
    botwfd_plugin.output_flags = OBS_SOURCE_VIDEO | OBS_SOURCE_ASYNC;
    botwfd_plugin.get_name = botwfdplugin_get_name;
    botwfd_plugin.create = botwfdplugin_create;
    botwfd_plugin.destroy = botwfdplugin_destroy;
    //.get_defaults = botwqt_get_defaults,
    //.get_properties = botwqt_get_properties,
    //.update = botwqt_update_settings,
    botwfd_plugin.filter_video = botwfdplugin_filter_video;

    obs_register_source(&botwfd_plugin);
    obs_log(LOG_INFO, "source registered");
    botwfd_load();
    obs_log(LOG_INFO, "core loaded");
    obs_log(LOG_INFO, "plugin loaded successfully)");
    return true;
}

extern "C" void obs_module_unload(void) {
    obs_log(LOG_INFO, "plugin unloaded");
}
