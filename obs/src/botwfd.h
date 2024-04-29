#pragma once

#include <obs-module.h>
#include <plugin-support.h>
#include "../../core/target/release/botwfdcore.h"

namespace botwfd {
    bool is_format_supported(enum video_format format);
    void filter_video(BotwFdCore* p_core, struct obs_source_frame *p_frame);
}