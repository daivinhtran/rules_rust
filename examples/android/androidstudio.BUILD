java_import(
    name = "jar_imports",
    jars = [
        "plugins/android/lib/sdk-tools.jar",
        "plugins/android/lib/android-base-common.jar",
        "lib/3rd-party-rt.jar",
        "plugins/android/lib/r8.jar",
        "plugins/android/lib/deploy_java_proto.jar",
        "plugins/android/lib/studio-proto.jar",
        "plugins/android/lib/libjava_sites.jar",
        "plugins/android/lib/libjava_version.jar",
    ],
)

java_binary(
    name = "deployer",
    main_class = "com.android.tools.deployer.DeployerRunner",
    visibility = ["//visibility:public"],
    runtime_deps = [
        ":jar_imports",
    ],
)
