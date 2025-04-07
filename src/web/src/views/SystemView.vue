<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { DashboardOutlined, DesktopOutlined, HddOutlined, ApiOutlined } from '@ant-design/icons-vue';
import { systemApi } from '@/api';
import { message } from 'ant-design-vue';

// 系统信息
const systemInfo = ref({
  os: {
    name: '',
    version: '',
    build: '',
    architecture: '',
  },
  hardware: {
    cpu: '',
    memory: {
      total: '',
      used: '',
      free: '',
    },
    disk: {
      c: {
        total: '',
        used: '',
        free: '',
      },
      d: {
        total: '',
        used: '',
        free: '',
      },
    },
    gpu: '',
  },
  network: {
    hostname: '',
    ipAddress: '',
    macAddress: '',
    isConnected: false,
  },
});

const loading = ref(true);
const cpuUsage = ref(0);
const memoryUsage = ref(0);
const diskUsage = ref(0);
const networkUsage = ref(0);

// 获取系统信息
const fetchSystemInfo = async () => {
  try {
    loading.value = true;
    const response = await systemApi.getSystemInfo();
    console.log('系统信息 API 响应:', response);
    
    // 检查响应数据结构
    const data = response.data;
    
    // 根据实际 API 返回结构映射数据
    systemInfo.value = {
      os: {
        name: data.os || '',
        version: data.version || '',
        build: data.build || '',
        architecture: data.architecture || '',
      },
      hardware: {
        cpu: data.cpu?.model || '',
        memory: {
          total: data.memory?.total || '',
          used: data.memory?.used || '',
          free: data.memory?.free || '',
        },
        disk: {
          c: {
            total: data.disk?.c?.total || '',
            used: data.disk?.c?.used || '',
            free: data.disk?.c?.free || '',
          },
          d: {
            total: data.disk?.d?.total || '',
            used: data.disk?.d?.used || '',
            free: data.disk?.d?.free || '',
          },
        },
        gpu: data.gpu || '',
      },
      network: {
        hostname: data.hostname || '',
        ipAddress: data.ipAddress || '',
        macAddress: data.macAddress || '',
        isConnected: data.isConnected || false,
      },
    };
    
    loading.value = false;
  } catch (error) {
    console.error('获取系统信息失败:', error);
    message.error('获取系统信息失败，请稍后重试');
    loading.value = false;
  }
};

// 获取系统资源使用情况
const fetchSystemResources = async () => {
  try {
    const response = await systemApi.getSystemResources();
    console.log('系统资源 API 响应:', response);
    
    // 检查响应数据结构
    const data = response.data;
    
    // 根据实际 API 返回结构映射数据
    cpuUsage.value = data.cpu_usage ? Math.round(data.cpu_usage) : 0;
    memoryUsage.value = data.memory_usage ? Math.round(data.memory_usage) : 0;
    diskUsage.value = data.disk_usage ? Math.round(data.disk_usage) : 0;
    networkUsage.value = data.network_usage ? Math.round(data.network_usage) : 0;
    
    // 定时更新资源使用情况
    setTimeout(fetchSystemResources, 5000);
  } catch (error) {
    console.error('获取系统资源使用情况失败:', error);
    // 如果获取失败，5秒后重试
    setTimeout(fetchSystemResources, 5000);
  }
};

onMounted(() => {
  // 获取系统信息和资源使用情况
  fetchSystemInfo();
  fetchSystemResources();
});

// 刷新系统信息
const refreshSystemInfo = () => {
  fetchSystemInfo();
};
</script>

<template>
  <div class="system-container">
    <a-row :gutter="16" class="page-header">
      <a-col :span="24">
        <a-page-header
          title="系统信息"
          subtitle="查看系统状态和资源使用情况"
        >
          <template #extra>
            <a-button type="primary" @click="refreshSystemInfo" :loading="loading">
              刷新
            </a-button>
          </template>
        </a-page-header>
      </a-col>
    </a-row>
    
    <!-- 资源使用情况 -->
    <a-row :gutter="16" class="resource-usage">
      <a-col :xs="24" :sm="12" :md="6">
        <a-card>
          <template #title>
            <DashboardOutlined /> CPU 使用率
          </template>
          <a-progress
            type="dashboard"
            :percent="cpuUsage"
            :format="(percent: number) => `${percent}%`"
            :strokeColor="{ '0%': '#108ee9', '100%': cpuUsage > 80 ? '#f5222d' : '#87d068' }"
          />
        </a-card>
      </a-col>
      
      <a-col :xs="24" :sm="12" :md="6">
        <a-card>
          <template #title>
            <HddOutlined /> 内存使用率
          </template>
          <a-progress
            type="dashboard"
            :percent="memoryUsage"
            :format="(percent: number) => `${percent}%`"
            :strokeColor="{ '0%': '#108ee9', '100%': memoryUsage > 80 ? '#f5222d' : '#87d068' }"
          />
        </a-card>
      </a-col>
      
      <a-col :xs="24" :sm="12" :md="6">
        <a-card>
          <template #title>
            <DesktopOutlined /> 磁盘使用率
          </template>
          <a-progress
            type="dashboard"
            :percent="diskUsage"
            :format="(percent: number) => `${percent}%`"
            :strokeColor="{ '0%': '#108ee9', '100%': diskUsage > 80 ? '#f5222d' : '#87d068' }"
          />
        </a-card>
      </a-col>
      
      <a-col :xs="24" :sm="12" :md="6">
        <a-card>
          <template #title>
            <ApiOutlined /> 网络使用率
          </template>
          <a-progress
            type="dashboard"
            :percent="networkUsage"
            :format="(percent: number) => `${percent}%`"
            :strokeColor="{ '0%': '#108ee9', '100%': '#87d068' }"
          />
        </a-card>
      </a-col>
    </a-row>
    
    <!-- 系统详细信息 -->
    <a-row :gutter="16" class="system-details">
      <a-col :span="24">
        <a-card title="系统详细信息" :loading="loading">
          <a-descriptions bordered>
            <a-descriptions-item label="操作系统" :span="3">
              {{ systemInfo.os.name }} ({{ systemInfo.os.version }})
            </a-descriptions-item>
            <a-descriptions-item label="系统版本">
              Build {{ systemInfo.os.build }}
            </a-descriptions-item>
            <a-descriptions-item label="系统架构">
              {{ systemInfo.os.architecture }}
            </a-descriptions-item>
            <a-descriptions-item label="主机名">
              {{ systemInfo.network.hostname }}
            </a-descriptions-item>
            <a-descriptions-item label="处理器" :span="3">
              {{ systemInfo.hardware.cpu }}
            </a-descriptions-item>
            <a-descriptions-item label="内存">
              总计: {{ systemInfo.hardware.memory.total }}
            </a-descriptions-item>
            <a-descriptions-item label="已用内存">
              {{ systemInfo.hardware.memory.used }}
            </a-descriptions-item>
            <a-descriptions-item label="可用内存">
              {{ systemInfo.hardware.memory.free }}
            </a-descriptions-item>
            <a-descriptions-item label="显卡" :span="3">
              {{ systemInfo.hardware.gpu }}
            </a-descriptions-item>
          </a-descriptions>
        </a-card>
      </a-col>
    </a-row>
    
    <!-- 磁盘信息 -->
    <a-row :gutter="16" class="disk-info">
      <a-col :span="24">
        <a-card title="磁盘信息" :loading="loading">
          <a-table
            :dataSource="[
              { key: 'c', drive: 'C:', total: systemInfo.hardware.disk.c.total, used: systemInfo.hardware.disk.c.used, free: systemInfo.hardware.disk.c.free },
              { key: 'd', drive: 'D:', total: systemInfo.hardware.disk.d.total, used: systemInfo.hardware.disk.d.used, free: systemInfo.hardware.disk.d.free }
            ]"
            :pagination="false"
          >
            <a-table-column title="驱动器" dataIndex="drive" key="drive" />
            <a-table-column title="总容量" dataIndex="total" key="total" />
            <a-table-column title="已用空间" dataIndex="used" key="used" />
            <a-table-column title="可用空间" dataIndex="free" key="free" />
            <a-table-column title="使用率" key="usage">
              <template #default="{ record }">
                <a-progress
                  :percent="Math.round(parseInt(record.used) / parseInt(record.total) * 100)"
                  :status="Math.round(parseInt(record.used) / parseInt(record.total) * 100) > 80 ? 'exception' : 'normal'"
                />
              </template>
            </a-table-column>
          </a-table>
        </a-card>
      </a-col>
    </a-row>
    
    <!-- 网络信息 -->
    <a-row :gutter="16" class="network-info">
      <a-col :span="24">
        <a-card title="网络信息" :loading="loading">
          <a-descriptions bordered>
            <a-descriptions-item label="IP 地址">
              {{ systemInfo.network.ipAddress }}
            </a-descriptions-item>
            <a-descriptions-item label="MAC 地址">
              {{ systemInfo.network.macAddress }}
            </a-descriptions-item>
            <a-descriptions-item label="连接状态">
              <a-tag :color="systemInfo.network.isConnected ? 'success' : 'error'">
                {{ systemInfo.network.isConnected ? '已连接' : '未连接' }}
              </a-tag>
            </a-descriptions-item>
          </a-descriptions>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<style scoped>
.system-container {
  padding: 20px;
}

.page-header {
  margin-bottom: 20px;
}

.resource-usage {
  margin-bottom: 20px;
}

.resource-usage .ant-card {
  margin-bottom: 16px;
  text-align: center;
}

.system-details,
.disk-info,
.network-info {
  margin-bottom: 20px;
}

.ant-progress-text {
  font-weight: bold;
}
</style>
