fn main() {
    // 检测目标环境并相应地嵌入资源
    if cfg!(target_env = "msvc") {
        // MSVC环境下使用标准的资源编译
        embed_resource::compile("resources.rc", embed_resource::NONE);
    } else if cfg!(target_env = "gnu") {
        // GNU环境下同样可以使用embed-resource，但可能需要额外配置
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
}