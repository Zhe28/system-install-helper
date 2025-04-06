<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { HomeOutlined, AppstoreOutlined, FileOutlined, DashboardOutlined } from '@ant-design/icons-vue';

const router = useRouter();
const selectedKeys = ref<string[]>(['home']);

const handleMenuClick = (key: string) => {
  router.push({ name: key });
  selectedKeys.value = [key];
};
</script>

<template>
  <a-layout class="layout">
    <a-layout-header class="header">
      <div class="logo">系统安装助手</div>
      <a-menu
        v-model:selectedKeys="selectedKeys"
        theme="dark"
        mode="horizontal"
        :style="{ lineHeight: '64px' }"
      >
        <a-menu-item key="home" @click="handleMenuClick('home')">
          <template #icon><HomeOutlined /></template>
          系统概览
        </a-menu-item>
        <a-menu-item key="software" @click="handleMenuClick('software')">
          <template #icon><AppstoreOutlined /></template>
          软件安装
        </a-menu-item>
        <a-menu-item key="config-files" @click="handleMenuClick('config-files')">
          <template #icon><FileOutlined /></template>
          配置文件
        </a-menu-item>
        <a-menu-item key="system" @click="handleMenuClick('system')">
          <template #icon><DashboardOutlined /></template>
          系统信息
        </a-menu-item>
      </a-menu>
    </a-layout-header>
    <a-layout-content class="content">
      <RouterView />
    </a-layout-content>
    <a-layout-footer class="footer">
      系统安装助手 2025 Created by Codeium
    </a-layout-footer>
  </a-layout>
</template>

<style scoped>
.layout {
  min-height: 100vh;
}

.header {
  display: flex;
  align-items: center;
}

.logo {
  color: white;
  font-size: 20px;
  font-weight: bold;
  margin-right: 30px;
}

.content {
  padding: 24px 50px;
  background: #f0f2f5;
}

.footer {
  text-align: center;
}
</style>
