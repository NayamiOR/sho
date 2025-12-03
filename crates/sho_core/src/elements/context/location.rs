use bon::Builder;

/// 地点
/// 界定历史事件发生的地理位置
/// WARNING: 地理方面方案还未确定
#[derive(Debug, Builder, Clone)]
pub struct Location {
    pub name: String,
    pub ancient_names: Option<Vec<String>>, // 别称/古称（可选）
    pub location_type: LocationType,        // 类型
    pub geographic_description: Option<String>, // 地理位置描述/大致坐标（可选）
    pub historical_evolution: Option<String>, // 历史沿革/描述（如：某地在不同时间属不同势力管辖）
}

/// 地点类型
#[derive(Debug, Clone)]
pub enum LocationType {
    Prefecture,    // 州郡
    City,          // 城池
    Pass,          // 关隘
    Battlefield,   // 战场
    Mountain,      // 山川
    River,         // 河流
    Other(String), // 其他类型
}
