set(headers ${headers}
    src/plugin/mcm_glue.h
    src/plugin/sinks.h
    src/PCH.h
)
set(sources ${sources}
    ${headers}
    src/main.cpp
    src/plugin/mcm_glue.cpp
    src/plugin/sinks.cpp
)
