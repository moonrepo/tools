#[cfg(not(windows))]
mod npm {
    use proto_pdk_test_utils::*;

    generate_shims_test!("npm-test", ["npm", "npx", "node-gyp"]);
}

#[cfg(not(windows))]
mod pnpm {
    use proto_pdk_test_utils::*;

    generate_shims_test!("pnpm-test", ["pnpm", "pnpx"]);
}

#[cfg(not(windows))]
mod yarn {
    use proto_pdk_test_utils::*;

    generate_shims_test!("yarn-test", ["yarn", "yarnpkg"]);
}
