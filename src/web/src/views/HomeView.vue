<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { AppstoreOutlined, FileOutlined, DashboardOutlined } from '@ant-design/icons-vue';

const router = useRouter();
const loading = ref(true);
const stats = ref({
  softwareCount: 0,
  configFilesCount: 0,
  systemHealth: '良好'
});

onMounted(() => {
  // 模拟加载数据
  setTimeout(() => {
    stats.value = {
      softwareCount: 45,
      configFilesCount: 12,
      systemHealth: '良好'
    };
    loading.value = false;
  }, 1000);
});

const navigateTo = (path: string) => {
  router.push(path);
};
</script>

<template>
  <div class="home-container">
    <a-row :gutter="16">
      <a-col :span="24">
        <a-typography-title>系统安装助手</a-typography-title>
        <a-typography-paragraph>
          欢迎使用系统安装助手，这是一个帮助您在重装系统后自动安装常用软件并恢复个人配置文件的工具。
        </a-typography-paragraph>
      </a-col>
    </a-row>

    <a-row :gutter="16" class="stat-cards">
      <a-col :xs="24" :sm="8">
        <a-card hoverable @click="navigateTo('/software')">
          <template #cover>
            <div class="card-icon-container">
              <AppstoreOutlined class="card-icon" />
            </div>
          </template>
          <a-card-meta title="软件管理">
            <template #description>
              <a-statistic
                title="可安装软件"
                :value="stats.softwareCount"
                :loading="loading"
              />
              <a-button type="primary" class="card-button">浏览软件</a-button>
            </template>
          </a-card-meta>
        </a-card>
      </a-col>

      <a-col :xs="24" :sm="8">
        <a-card hoverable @click="navigateTo('/config-files')">
          <template #cover>
            <div class="card-icon-container">
              <FileOutlined class="card-icon" />
            </div>
          </template>
          <a-card-meta title="配置文件">
            <template #description>
              <a-statistic
                title="配置文件数量"
                :value="stats.configFilesCount"
                :loading="loading"
              />
              <a-button type="primary" class="card-button">管理配置</a-button>
            </template>
          </a-card-meta>
        </a-card>
      </a-col>

      <a-col :xs="24" :sm="8">
        <a-card hoverable @click="navigateTo('/system')">
          <template #cover>
            <div class="card-icon-container">
              <DashboardOutlined class="card-icon" />
            </div>
          </template>
          <a-card-meta title="系统信息">
            <template #description>
              <a-statistic
                title="系统健康状态"
                :value="stats.systemHealth"
                :loading="loading"
              />
              <a-button type="primary" class="card-button">查看详情</a-button>
            </template>
          </a-card-meta>
        </a-card>
      </a-col>
    </a-row>

    <a-row :gutter="16" class="quick-start">
      <a-col :span="24">
        <a-card title="快速开始">
          <a-steps>
            <a-step title="选择软件" description="从软件列表中选择您需要安装的软件" />
            <a-step title="配置文件" description="设置您的个人配置文件" />
            <a-step title="开始安装" description="一键安装所有选定的软件并恢复配置" />
          </a-steps>
          <div class="start-button-container">
            <a-button type="primary" size="large" @click="navigateTo('/software')">
              立即开始
            </a-button>
          </div>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<style scoped>
.home-container {
  padding: 20px;
}

.stat-cards {
  margin-top: 20px;
  margin-bottom: 20px;
}

.card-icon-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 120px;
  background-color: #f5f5f5;
}

.card-icon {
  font-size: 48px;
  color: #1890ff;
}

.card-button {
  margin-top: 16px;
  width: 100%;
}

.quick-start {
  margin-top: 20px;
}

.start-button-container {
  display: flex;
  justify-content: center;
  margin-top: 24px;
}
</style>
