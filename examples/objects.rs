//! 上传文件

use tencent_qcloud_cos_rs::acl::{AclHeader, ObjectAcl};
use tencent_qcloud_cos_rs::client::Client;
use tencent_qcloud_cos_rs::objects::{mime, Objects};
use tencent_qcloud_cos_rs::request::ErrNo;

#[tokio::main]
async fn main() {
    let client = Client::new(
        "Your secrect id",
        "Your secrect key",
        Some(String::from("Your security token")),
        "bucket name",
        "region",
    );
    // 普通上传，无权限控制
    let res = client
        .put_object("Cargo.toml", "Cargo.toml", mime::TEXT_PLAIN_UTF_8, None)
        .await;
    if res.error_no == ErrNo::SUCCESS {
        println!("SUCCESS");
    } else {
        println!("{}", res.error_message);
    }
    // 私有权限控制
    let mut acl = AclHeader::new();
    acl.insert_object_x_cos_acl(ObjectAcl::PRIVATE);
    let res = client
        .put_object(
            "Cargo.toml",
            "Cargo.toml",
            mime::TEXT_PLAIN_UTF_8,
            Some(&acl),
        )
        .await;
    if res.error_no == ErrNo::SUCCESS {
        println!("SUCCESS");
    } else {
        println!("{}", res.error_message);
    }
    // 分块传输
    let res = client
        .put_big_object(
            "Cargo.toml",
            "Cargo.toml",
            mime::TEXT_PLAIN_UTF_8,
            "ARCHIVE",
            None,
            1024,
        )
        .await;
    if res.error_no == ErrNo::SUCCESS {
        println!("SUCCESS");
    } else {
        println!("{}", res.error_message);
    }
    // 直接上传二进制流
    let res = client
        .put_object_binary(
            std::fs::read("Cargo.toml").unwrap(),
            "Cargo.toml",
            mime::TEXT_PLAIN_UTF_8,
            None,
        )
        .await;
    if res.error_no == ErrNo::SUCCESS {
        println!("SUCCESS");
    } else {
        println!("{}", res.error_message);
    }
    // 删除文件 test/Cargo.toml
    let res = client.delete_object("test/Cargo.toml").await;
    if res.error_no == ErrNo::SUCCESS {
        println!("SUCCESS");
    } else {
        println!("{}", res.error_message);
    }
    // 将对象存储对象名称为Cargo.toml的文件下载到本地，名称为local_Cargo.toml
    let res = client.get_object("Cargo.toml", "local_Cargo.toml").await;
    if res.error_no == ErrNo::SUCCESS {
        println!("SUCCESS");
    } else {
        println!("{}", res.error_message);
    }

    // 获取预签名下载URL
    let url = client.get_presigned_download_url("Cargo.toml", 3600);
    println!("full_url: {}", url);
}
