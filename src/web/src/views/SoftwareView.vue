<script setup lang="ts">
import { ref, reactive, onMounted, computed, h } from 'vue';
import { SearchOutlined, DownloadOutlined, SettingOutlined, AppstoreOutlined } from '@ant-design/icons-vue';
import { Tag, Button, message } from 'ant-design-vue';
import { softwareApi } from '../api';

// 软件分类
const categories = [
  { key: 'all', name: '全部' },
  { key: 'dev', name: '开发工具' },
  { key: 'office', name: '办公软件' },
  { key: 'browser', name: '浏览器' },
  { key: 'media', name: '多媒体' },
  { key: 'utility', name: '实用工具' },
  { key: 'game', name: '游戏' },
];

// 软件列表
const softwareList = ref<any[]>([]);
const loading = ref(true);
const searchValue = ref('');
const selectedCategory = ref('all');
const selectedRowKeys = ref<string[]>([]);
const installLoading = ref(false);
const pagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

// 模拟软件数据
const mockSoftwareData = [
  {
    id: '1',
    name: 'Visual Studio Code',
    category: 'dev',
    version: '1.85.0',
    size: '80MB',
    description: '轻量级代码编辑器',
    publisher: 'Microsoft',
    installStatus: 'notInstalled',
    icon: '🧰',
  },
  {
    id: '2',
    name: 'Google Chrome',
    category: 'browser',
    version: '120.0.6099.129',
    size: '85MB',
    description: '快速、安全的网络浏览器',
    publisher: 'Google',
    installStatus: 'installed',
    icon: '🌐',
  },
  {
    id: '3',
    name: 'Microsoft Office',
    category: 'office',
    version: '2021',
    size: '4.2GB',
    description: '办公软件套件',
    publisher: 'Microsoft',
    installStatus: 'notInstalled',
    icon: '📊',
  },
  {
    id: '4',
    name: 'Adobe Photoshop',
    category: 'media',
    version: '25.0',
    size: '2.5GB',
    description: '专业图像编辑软件',
    publisher: 'Adobe',
    installStatus: 'notInstalled',
    icon: '🖼️',
  },
  {
    id: '5',
    name: '7-Zip',
    category: 'utility',
    version: '23.01',
    size: '1.5MB',
    description: '高效的文件压缩工具',
    publisher: 'Igor Pavlov',
    installStatus: 'notInstalled',
    icon: '🗜️',
  },
  {
    id: '6',
    name: 'Steam',
    category: 'game',
    version: '2.10.91.91',
    size: '250MB',
    description: '游戏平台',
    publisher: 'Valve',
    installStatus: 'notInstalled',
    icon: '🎮',
  },
  {
    id: '7',
    name: 'Node.js',
    category: 'dev',
    version: '20.10.0',
    size: '32MB',
    description: 'JavaScript 运行时',
    publisher: 'OpenJS Foundation',
    installStatus: 'notInstalled',
    icon: '⚙️',
  },
  {
    id: '8',
    name: 'Firefox',
    category: 'browser',
    version: '121.0',
    size: '55MB',
    description: '注重隐私的网络浏览器',
    publisher: 'Mozilla',
    installStatus: 'notInstalled',
    icon: '🦊',
  },
  {
    id: '9',
    name: 'VLC Media Player',
    category: 'media',
    version: '3.0.20',
    size: '40MB',
    description: '多功能媒体播放器',
    publisher: 'VideoLAN',
    installStatus: 'installed',
    icon: '🎬',
  },
  {
    id: '10',
    name: 'WinRAR',
    category: 'utility',
    version: '6.23',
    size: '3MB',
    description: '压缩文件管理器',
    publisher: 'win.rar GmbH',
    installStatus: 'notInstalled',
    icon: '📦',
  },
];

// 获取软件列表
const fetchSoftwareList = async () => {
  loading.value = true;
  try {
    const response = await softwareApi.getAllSoftware();
    if (response.data) {
      // 转换后端数据格式为前端格式
      softwareList.value = response.data.map((item: any) => ({
        id: item.id || item.name.toLowerCase().replace(/\s+/g, '-'),
        name: item.name,
        category: mapCategoryToFrontend(item.category),
        version: item.version,
        size: item.size || '未知',
        description: item.description || '暂无描述',
        publisher: item.publisher || '未知',
        installStatus: item.installed ? 'installed' : 'notInstalled',
        icon: getCategoryIcon(item.category),
      }));
    }
  } catch (error) {
    console.error('获取软件列表失败:', error);
    message.error('获取软件列表失败，使用本地数据');
    // 使用模拟数据作为备用
    softwareList.value = mockSoftwareData;
  } finally {
    loading.value = false;
  }
};

// 将后端分类映射到前端分类
const mapCategoryToFrontend = (category: string) => {
  const categoryMap: Record<string, string> = {
    'Development': 'dev',
    'Communication': 'browser',
    'Office': 'office',
    'Media': 'media',
    'Utility': 'utility',
    'Game': 'game',
  };
  return categoryMap[category] || 'utility';
};

// 根据分类获取图标
const getCategoryIcon = (category: string) => {
  const iconMap: Record<string, string> = {
    'Development': '🧰',
    'Communication': '🌐',
    'Office': '📊',
    'Media': '🖼️',
    'Utility': '🗜️',
    'Game': '🎮',
  };
  return iconMap[category] || '📦';
};

// 处理安装
const handleInstall = (record: any) => {
  // 实际调用API进行安装
  message.loading(`正在安装 ${record.name}...`, 2.5);
  softwareApi.installSoftware(record.id)
    .then(() => {
      message.success(`${record.name} 安装成功！`);
      // 更新软件状态
      record.installStatus = 'installed';
    })
    .catch((error) => {
      console.error('安装失败:', error);
      message.error(`${record.name} 安装失败！`);
    });
};

// 处理卸载
const handleUninstall = (record: any) => {
  // 实际调用API进行卸载
  message.loading(`正在卸载 ${record.name}...`, 2.5);
  softwareApi.uninstallSoftware(record.id)
    .then(() => {
      message.success(`${record.name} 卸载成功！`);
      // 更新软件状态
      record.installStatus = 'notInstalled';
    })
    .catch((error) => {
      console.error('卸载失败:', error);
      message.error(`${record.name} 卸载失败！`);
    });
};

// 批量安装
const handleBatchInstall = () => {
  if (selectedRowKeys.value.length === 0) {
    message.warning('请先选择要安装的软件');
    return;
  }
  
  installLoading.value = true;
  message.loading(`正在批量安装 ${selectedRowKeys.value.length} 个软件...`, 2.5);
  
  // 实际应用中这里会调用API进行批量安装
  const installPromises = selectedRowKeys.value.map(id => 
    softwareApi.installSoftware(id as string)
  );
  
  Promise.all(installPromises)
    .then(() => {
      message.success('批量安装成功！');
      // 更新软件状态
      softwareList.value.forEach(item => {
        if (selectedRowKeys.value.includes(item.id)) {
          item.installStatus = 'installed';
        }
      });
      selectedRowKeys.value = [];
    })
    .catch((error) => {
      console.error('批量安装失败:', error);
      message.error('批量安装失败！');
    })
    .finally(() => {
      installLoading.value = false;
    });
};

// 处理搜索
const handleSearch = () => {
  if (!searchValue.value) {
    return;
  }
  
  loading.value = true;
  softwareApi.searchSoftware(searchValue.value, selectedCategory.value)
    .then(response => {
      if (response.data) {
        // 转换后端数据格式为前端格式
        softwareList.value = response.data.map((item: any) => ({
          id: item.id || item.name.toLowerCase().replace(/\s+/g, '-'),
          name: item.name,
          category: mapCategoryToFrontend(item.category),
          version: item.version,
          size: item.size || '未知',
          description: item.description || '暂无描述',
          publisher: item.publisher || '未知',
          installStatus: item.installed ? 'installed' : 'notInstalled',
          icon: getCategoryIcon(item.category),
        }));
      }
    })
    .catch(error => {
      console.error('搜索失败:', error);
      message.error('搜索失败');
    })
    .finally(() => {
      loading.value = false;
    });
};

// 处理分类切换
const handleCategoryChange = (category: string) => {
  selectedCategory.value = category;
};

// 过滤软件列表
const filteredSoftwareList = computed(() => {
  let result = [...softwareList.value];
  
  // 按分类过滤
  if (selectedCategory.value !== 'all') {
    result = result.filter(item => item.category === selectedCategory.value);
  }
  
  // 按搜索关键词过滤
  if (searchValue.value) {
    const keyword = searchValue.value.toLowerCase();
    result = result.filter(item => 
      item.name.toLowerCase().includes(keyword) || 
      item.description.toLowerCase().includes(keyword) ||
      item.publisher.toLowerCase().includes(keyword)
    );
  }
  
  return result;
});

// 表格列定义
interface Column {
  title: string;
  dataIndex: string;
  key: string;
  width?: number;
  customRender?: (params: { text: string; record: any }) => any;
}

const columns: Column[] = [
  {
    title: '',
    dataIndex: 'icon',
    key: 'icon',
    width: 60,
  },
  {
    title: '软件名称',
    dataIndex: 'name',
    key: 'name',
  },
  {
    title: '版本',
    dataIndex: 'version',
    key: 'version',
    width: 120,
  },
  {
    title: '大小',
    dataIndex: 'size',
    key: 'size',
    width: 100,
  },
  {
    title: '发布商',
    dataIndex: 'publisher',
    key: 'publisher',
  },
  {
    title: '状态',
    dataIndex: 'installStatus',
    key: 'installStatus',
    width: 120,
    customRender: ({ text }: { text: string }) => {
      if (text === 'installed') {
        return h(Tag, { color: 'success' }, () => '已安装');
      } else {
        return h(Tag, { color: 'default' }, () => '未安装');
      }
    },
  },
  {
    title: '操作',
    key: 'action',
    width: 120,
    customRender: ({ record }: { record: any; }) => {
      if (record.installStatus === 'installed') {
        return h(Button, {
          type: 'primary',
          danger: true,
          size: 'small',
          onClick: () => handleUninstall(record),
        }, () => '卸载');
      } else {
        return h(Button, {
          type: 'primary',
          size: 'small',
          onClick: () => handleInstall(record),
        }, () => '安装');
      }
    },
    dataIndex: 'action',
  },
];

// 表格行选择配置
const rowSelection = {
  selectedRowKeys: selectedRowKeys.value,
  onChange: (keys: string[]) => {
    selectedRowKeys.value = keys;
  },
};

// 初始化
onMounted(() => {
  fetchSoftwareList();
});
</script>

<template>
  <div class="software-container">
    <a-row :gutter="16" class="page-header">
      <a-col :span="24">
        <a-page-header
          title="软件安装"
          subtitle="选择并安装您需要的软件"
        />
      </a-col>
    </a-row>
    
    <a-row :gutter="16" class="search-bar">
      <a-col :xs="24" :sm="16" :md="18">
        <a-input-search
          v-model:value="searchValue"
          placeholder="搜索软件名称、描述或发布商"
          enter-button
          @search="handleSearch"
        >
          <template #prefix>
            <SearchOutlined />
          </template>
        </a-input-search>
      </a-col>
      <a-col :xs="24" :sm="8" :md="6" class="action-buttons">
        <a-button
          type="primary"
          :loading="installLoading"
          :disabled="selectedRowKeys.length === 0"
          @click="handleBatchInstall"
        >
          <template #icon><DownloadOutlined /></template>
          批量安装 ({{ selectedRowKeys.length }})
        </a-button>
      </a-col>
    </a-row>
    
    <a-row :gutter="16" class="content-area">
      <a-col :xs="24" :sm="6" :md="5" :lg="4" class="category-menu">
        <a-menu
          :selectedKeys="[selectedCategory]"
          mode="inline"
          style="height: 100%"
        >
          <a-menu-item v-for="category in categories" :key="category.key" @click="handleCategoryChange(category.key)">
            <template #icon>
              <AppstoreOutlined v-if="category.key === 'all'" />
              <SettingOutlined v-else />
            </template>
            {{ category.name }}
          </a-menu-item>
        </a-menu>
      </a-col>
      
      <a-col :xs="24" :sm="18" :md="19" :lg="20" class="software-list">
        <a-table
          :columns="columns"
          :data-source="filteredSoftwareList"
          :row-selection="rowSelection"
          :pagination="pagination"
          :loading="loading"
          row-key="id"
        >
          <template #expandedRowRender="{ record }">
            <p style="margin: 0">{{ record.description }}</p>
          </template>
        </a-table>
      </a-col>
    </a-row>
  </div>
</template>

<style scoped>
.software-container {
  padding: 20px;
}

.page-header {
  margin-bottom: 20px;
}

.search-bar {
  margin-bottom: 20px;
}

.action-buttons {
  display: flex;
  justify-content: flex-end;
}

.content-area {
  background: #fff;
  border-radius: 4px;
}

.category-menu {
  background: #fff;
  border-right: 1px solid #f0f0f0;
}

.software-list {
  min-height: 500px;
}

@media (max-width: 576px) {
  .action-buttons {
    margin-top: 16px;
    justify-content: flex-start;
  }
}
</style>
