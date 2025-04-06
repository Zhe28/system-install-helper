<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { DashboardOutlined, DesktopOutlined, HddOutlined, ApiOutlined } from '@ant-design/icons-vue';

// 系统信息
const systemInfo = ref({
  os: {
    name: 'Windows 11 专业版',
    version: '22H2',
    build: '22621.3155',
    architecture: 'x64',
  },
  hardware: {
    cpu: 'Intel Core i7-12700K',
    memory: {
      total: '32GB',
      used: '12.5GB',
      free: '19.5GB',
    },
    disk: {
      c: {
        total: '512GB',
        used: '256GB',
        free: '256GB',
      },
      d: {
        total: '1TB',
        used: '350GB',
        free: '650GB',
      },
    },
    gpu: 'NVIDIA GeForce RTX 3080',
  },
  network: {
    hostname: 'DESKTOP-ABC123',
    ipAddress: '192.168.1.100',
    macAddress: '00:11:22:33:44:55',
    isConnected: true,
  },
});

const loading = ref(true);
const cpuUsage = ref(0);
const memoryUsage = ref(0);
const diskUsage = ref(0);
const networkUsage = ref(0);

// 模拟加载系统信息
onMounted(() => {
  // 模拟API请求
  setTimeout(() => {
    loading.value = false;
    
    // 模拟资源使用率数据
    cpuUsage.value = 35;
    memoryUsage.value = 39;
    diskUsage.value = 50;
    networkUsage.value = 15;
    
    // 模拟实时更新
    setInterval(() => {
      cpuUsage.value = Math.floor(25 + Math.random() * 30);
      memoryUsage.value = Math.floor(35 + Math.random() * 15);
      networkUsage.value = Math.floor(5 + Math.random() * 25);
    }, 3000);
  }, 1500);
});

// 刷新系统信息
const refreshSystemInfo = () => {
  loading.value = true;
  
  // 模拟API请求
  setTimeout(() => {
    loading.value = false;
    // 实际应用中这里会重新获取系统信息
  }, 1000);
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
            :format="percent => `${percent}%`"
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
            :format="percent => `${percent}%`"
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
            :format="percent => `${percent}%`"
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
            :format="percent => `${percent}%`"
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
