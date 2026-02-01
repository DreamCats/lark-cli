use super::ApiClient;
use crate::error::{LarkError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 用户信息响应结构
#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfoResponse {
    /// 用户信息
    pub user: UserInfo,
}

/// 用户详细信息
#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo {
    /// 用户的 union_id
    #[serde(rename = "union_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    /// 用户的 user_id
    #[serde(rename = "user_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户的 open_id
    #[serde(rename = "open_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 英文名
    #[serde(rename = "en_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// 邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 手机号码是否可见
    #[serde(rename = "mobile_visible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_visible: Option<bool>,
    /// 性别 (0:保密, 1:男, 2:女, 3:其他)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// 用户头像信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<AvatarInfo>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 用户所属部门的 ID 列表
    #[serde(rename = "department_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 用户的直接主管的用户ID
    #[serde(rename = "leader_user_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
    /// 工作城市
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 国家或地区 Code 缩写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// 工位
    #[serde(rename = "work_station")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_station: Option<String>,
    /// 入职时间（秒级时间戳）
    #[serde(rename = "join_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<i64>,
    /// 用户是否为租户超级管理员
    #[serde(rename = "is_tenant_manager")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tenant_manager: Option<bool>,
    /// 工号
    #[serde(rename = "employee_no")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_no: Option<String>,
    /// 员工类型 (1:正式员工, 2:实习生, 3:外包, 4:劳务, 5:顾问)
    #[serde(rename = "employee_type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<i32>,
    /// 用户排序信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<UserOrder>>,
    /// 自定义字段
    #[serde(rename = "custom_attrs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attrs: Option<Vec<UserCustomAttr>>,
    /// 企业邮箱
    #[serde(rename = "enterprise_email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_email: Option<String>,
    /// 职务
    #[serde(rename = "job_title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// 数据驻留地
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<String>,
    /// 职级 ID
    #[serde(rename = "job_level_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 序列 ID
    #[serde(rename = "job_family_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_family_id: Option<String>,
    /// 用户席位列表
    #[serde(rename = "assign_info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_info: Option<Vec<UserAssignInfo>>,
    /// 部门路径列表
    #[serde(rename = "department_path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_path: Option<Vec<DepartmentDetail>>,
    /// 虚线上级的用户 ID
    #[serde(rename = "dotted_line_leader_user_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dotted_line_leader_user_ids: Option<Vec<String>>,
}

/// 用户头像信息
#[derive(Debug, Deserialize, Serialize)]
pub struct AvatarInfo {
    /// 72*72 像素头像链接
    #[serde(rename = "avatar_72")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_72: Option<String>,
    /// 240*240 像素头像链接
    #[serde(rename = "avatar_240")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_240: Option<String>,
    /// 640*640 像素头像链接
    #[serde(rename = "avatar_640")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_640: Option<String>,
    /// 原始头像链接
    #[serde(rename = "avatar_origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_origin: Option<String>,
}

/// 用户状态
#[derive(Debug, Deserialize, Serialize)]
pub struct UserStatus {
    /// 是否为暂停状态
    #[serde(rename = "is_frozen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_frozen: Option<bool>,
    /// 是否为离职状态
    #[serde(rename = "is_resigned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resigned: Option<bool>,
    /// 是否为激活状态
    #[serde(rename = "is_activated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_activated: Option<bool>,
    /// 是否为主动退出状态
    #[serde(rename = "is_exited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_exited: Option<bool>,
    /// 是否为未加入状态
    #[serde(rename = "is_unjoin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unjoin: Option<bool>,
}

/// 用户排序信息
#[derive(Debug, Deserialize, Serialize)]
pub struct UserOrder {
    /// 排序信息对应的部门 ID
    #[serde(rename = "department_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 用户在其直属部门内的排序
    #[serde(rename = "user_order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_order: Option<i32>,
    /// 用户所属的多个部门间的排序
    #[serde(rename = "department_order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_order: Option<i32>,
    /// 标识是否为用户的唯一主部门
    #[serde(rename = "is_primary_dept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_dept: Option<bool>,
}

/// 自定义字段
#[derive(Debug, Deserialize, Serialize)]
pub struct UserCustomAttr {
    /// 自定义字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 自定义字段 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 自定义字段取值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<UserCustomAttrValue>,
}

/// 自定义字段取值
#[derive(Debug, Deserialize, Serialize)]
pub struct UserCustomAttrValue {
    /// TEXT 类型字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// PC 端 URL
    #[serde(rename = "pc_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    /// 枚举类型中选项的选项值
    #[serde(rename = "option_value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_value: Option<String>,
    /// 图片类型中图片选项的名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 图片类型中图片选项的链接
    #[serde(rename = "picture_url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    /// 引用人员
    #[serde(rename = "generic_user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_user: Option<CustomAttrGenericUser>,
}

/// 自定义字段引用人员
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomAttrGenericUser {
    /// 引用人员的用户 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 用户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<i32>,
}

/// 用户席位信息
#[derive(Debug, Deserialize, Serialize)]
pub struct UserAssignInfo {
    /// 席位 ID
    #[serde(rename = "subscription_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// 席位许可（License Plan Key）
    #[serde(rename = "license_plan_key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_plan_key: Option<String>,
    /// 席位名称
    #[serde(rename = "product_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// 国际化名称
    #[serde(rename = "i18n_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<ProductI18nName>,
    /// 席位起始时间
    #[serde(rename = "start_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 席位结束时间
    #[serde(rename = "end_time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
}

/// 国际化名称
#[derive(Debug, Deserialize, Serialize)]
pub struct ProductI18nName {
    /// 席位中文名
    #[serde(rename = "zh_cn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 席位日文名
    #[serde(rename = "ja_jp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    /// 席位英文名
    #[serde(rename = "en_us")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 部门路径详情
#[derive(Debug, Deserialize, Serialize)]
pub struct DepartmentDetail {
    /// 部门 ID
    #[serde(rename = "department_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称信息
    #[serde(rename = "department_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<DepartmentPathName>,
    /// 部门路径
    #[serde(rename = "department_path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_path: Option<DepartmentPath>,
}

/// 部门名称信息
#[derive(Debug, Deserialize, Serialize)]
pub struct DepartmentPathName {
    /// 部门名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 部门国际化名
    #[serde(rename = "i18n_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_name: Option<DepartmentI18nName>,
}

/// 部门国际化名
#[derive(Debug, Deserialize, Serialize)]
pub struct DepartmentI18nName {
    /// 部门的中文名
    #[serde(rename = "zh_cn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 部门的日文名
    #[serde(rename = "ja_jp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja_jp: Option<String>,
    /// 部门的英文名
    #[serde(rename = "en_us")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 部门路径
#[derive(Debug, Deserialize, Serialize)]
pub struct DepartmentPath {
    /// 部门路径 ID 列表
    #[serde(rename = "department_ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 部门路径名字信息
    #[serde(rename = "department_path_name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_path_name: Option<DepartmentPathName>,
}

pub struct GetUserInfoApi {
    client: ApiClient,
}

impl GetUserInfoApi {
    pub fn new(client: ApiClient) -> Self {
        Self { client }
    }

    /// 获取单个用户信息
    ///
    /// # 参数
    /// * `user_id` - 用户ID
    /// * `user_id_type` - 用户ID类型 (open_id/union_id/user_id)，默认 open_id
    /// * `department_id_type` - 部门ID类型 (department_id/open_department_id)，默认 open_department_id
    pub async fn get_user_info(
        &self,
        user_id: &str,
        user_id_type: Option<&str>,
        department_id_type: Option<&str>,
    ) -> Result<UserInfo> {
        // 验证参数
        if user_id.is_empty() {
            return Err(LarkError::ValidationError(
                "user_id 参数是必需的".to_string(),
            ));
        }

        // 验证 user_id_type
        if let Some(id_type) = user_id_type {
            let valid_types = ["open_id", "union_id", "user_id"];
            if !valid_types.contains(&id_type) {
                return Err(LarkError::ValidationError(format!(
                    "user_id_type 必须是以下值之一：{}",
                    valid_types.join(", ")
                )));
            }
        }

        // 验证 department_id_type
        if let Some(dept_type) = department_id_type {
            let valid_types = ["department_id", "open_department_id"];
            if !valid_types.contains(&dept_type) {
                return Err(LarkError::ValidationError(format!(
                    "department_id_type 必须是以下值之一：{}",
                    valid_types.join(", ")
                )));
            }
        }

        // 构建查询参数
        let mut params = HashMap::new();
        if let Some(id_type) = user_id_type {
            params.insert("user_id_type".to_string(), id_type.to_string());
        }
        if let Some(dept_type) = department_id_type {
            params.insert("department_id_type".to_string(), dept_type.to_string());
        }

        // 构建URL
        let url = format!(
            "https://open.larkoffice.com/open-apis/contact/v3/users/{}",
            user_id
        );

        // 发送请求
        let data: UserInfoResponse = self.client.get(&url, Some(params)).await?;

        Ok(data.user)
    }
}
