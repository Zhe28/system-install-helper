import axios from 'axios';

// 创建axios实例
const api = axios.create({
  baseURL: 'http://localhost:8080/api', // 后端API地址
  timeout: 10000, // 请求超时时间
  headers: {
    'Content-Type': 'application/json',
  },
});

// 软件相关API
export const softwareApi = {
  // 获取所有软件
  getAllSoftware: () => api.get('/software'),
  
  // 获取特定软件详情
  getSoftwareById: (id: string) => api.get(`/software/${id}`),
  
  // 安装软件
  installSoftware: (id: string, options?: any) => api.post('/software/install', { id, options }),
  
  // 卸载软件
  uninstallSoftware: (id: string) => api.delete(`/software/${id}`),
  
  // 获取安装状态
  getInstallStatus: (id: string) => api.get(`/software/status/${id}`),
  
  // 搜索软件
  searchSoftware: (query: string, category?: string) => {
    const params: any = { query };
    if (category && category !== 'all') {
      params.category = category;
    }
    return api.get('/software/search', { params });
  },
};

// 配置文件相关API
export const configFilesApi = {
  // 获取所有配置文件
  getAllConfigFiles: () => api.get('/config-files'),
  
  // 获取特定配置文件详情
  getConfigFileById: (id: string) => api.get(`/config-files/${id}`),
  
  // 部署配置文件
  deployConfigFile: (id: string) => api.post(`/config-files/${id}/deploy`),
  
  // 添加配置文件
  addConfigFile: (data: any) => api.post('/config-files', data),
  
  // 更新配置文件
  updateConfigFile: (id: string, data: any) => api.put(`/config-files/${id}`, data),
  
  // 删除配置文件
  deleteConfigFile: (id: string) => api.delete(`/config-files/${id}`),
};

// 系统信息相关API
export const systemApi = {
  // 获取系统信息
  getSystemInfo: () => api.get('/system/info'),
  
  // 获取系统资源使用情况
  getSystemResources: () => api.get('/system/status'),
};

export default {
  softwareApi,
  configFilesApi,
  systemApi,
};
