<script setup lang="ts">
import { ref, onMounted, computed, h } from 'vue';
import { FileAddOutlined, FolderOpenOutlined, SaveOutlined, DeleteOutlined } from '@ant-design/icons-vue';
import { message, Tag, Button } from 'ant-design-vue';
import { configFilesApi } from '../api';

// 配置文件列表
const configFilesList = ref<any[]>([]);
const loading = ref(true);
const searchValue = ref('');
const selectedRowKeys = ref<string[]>([]);
const deployLoading = ref(false);
const modalVisible = ref(false);
const currentFile = ref<any>(null);

// 模拟配置文件数据
const mockConfigFilesData = [
  {
    id: '1',
    name: 'VSCode 设置',
    path: '%APPDATA%\\Code\\User\\settings.json',
    description: 'Visual Studio Code 用户设置',
    lastModified: '2025-03-15 14:30:22',
    size: '8.2KB',
    type: 'editor',
  },
  {
    id: '2',
    name: 'Git 配置',
    path: '%USERPROFILE%\\.gitconfig',
    description: 'Git 全局配置文件',
    lastModified: '2025-02-20 09:15:43',
    size: '1.5KB',
    type: 'dev',
  },
  {
    id: '3',
    name: 'Windows Terminal 设置',
    path: '%LOCALAPPDATA%\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe\\LocalState\\settings.json',
    description: 'Windows Terminal 配置文件',
    lastModified: '2025-04-01 18:22:10',
    size: '12.7KB',
    type: 'terminal',
  },
  {
    id: '4',
    name: 'PowerShell 配置',
    path: '%USERPROFILE%\\Documents\\PowerShell\\Microsoft.PowerShell_profile.ps1',
    description: 'PowerShell 用户配置文件',
    lastModified: '2025-03-28 11:05:37',
    size: '3.8KB',
    type: 'terminal',
  },
  {
    id: '5',
    name: 'Notepad++ 配置',
    path: '%APPDATA%\\Notepad++\\config.xml',
    description: 'Notepad++ 配置文件',
    lastModified: '2025-01-10 15:42:19',
    size: '25.4KB',
    type: 'editor',
  },
  {
    id: '6',
    name: 'Chrome 书签',
    path: '%LOCALAPPDATA%\\Google\\Chrome\\User Data\\Default\\Bookmarks',
    description: 'Google Chrome 浏览器书签',
    lastModified: '2025-04-05 20:18:55',
    size: '1.2MB',
    type: 'browser',
  },
];

// 获取配置文件列表
const fetchConfigFiles = async () => {
  loading.value = true;
  try {
    const response = await configFilesApi.getAllConfigFiles();
    if (response.data) {
      // 转换后端数据格式为前端格式
      configFilesList.value = response.data.map((item: any) => ({
        id: item.id || item.name.toLowerCase().replace(/\s+/g, '-'),
        name: item.name,
        type: item.type || '配置文件',
        path: item.path || '/home/user/.config',
        description: item.description || '暂无描述',
        lastModified: item.lastModified || '2025-04-01',
      }));
    }
  } catch (error) {
    console.error('获取配置文件列表失败:', error);
    message.error('获取配置文件列表失败，使用本地数据');
    // 使用模拟数据作为备用
    configFilesList.value = mockConfigFilesData;
  } finally {
    loading.value = false;
  }
};

// 过滤配置文件列表
const filteredConfigFilesList = computed(() => {
  if (!searchValue.value) {
    return configFilesList.value;
  }
  
  const keyword = searchValue.value.toLowerCase();
  return configFilesList.value.filter(item => 
    item.name.toLowerCase().includes(keyword) || 
    item.description.toLowerCase().includes(keyword) ||
    item.path.toLowerCase().includes(keyword)
  );
});

// 表格列定义
const columns = [
  {
    title: '配置文件名称',
    dataIndex: 'name',
    key: 'name',
  },
  {
    title: '路径',
    dataIndex: 'path',
    key: 'path',
    ellipsis: true,
  },
  {
    title: '类型',
    dataIndex: 'type',
    key: 'type',
    width: 100,
    customRender: ({ text }: { text: string }) => {
      const typeMap: Record<string, { color: string; text: string }> = {
        'editor': { color: 'blue', text: '编辑器' },
        'dev': { color: 'green', text: '开发工具' },
        'terminal': { color: 'purple', text: '终端' },
        'browser': { color: 'orange', text: '浏览器' },
      };
      const type = typeMap[text] || { color: 'default', text };
      return h(Tag, { color: type.color }, () => type.text);
    },
  },
  {
    title: '大小',
    dataIndex: 'size',
    key: 'size',
    width: 100,
  },
  {
    title: '最后修改',
    dataIndex: 'lastModified',
    key: 'lastModified',
    width: 180,
  },
  {
    title: '操作',
    key: 'action',
    width: 180,
    customRender: ({ record }: { record: any }) => {
      return [
        h(Button, {
          type: 'primary',
          size: 'small',
          style: { marginRight: '8px' },
          onClick: () => handleDeploy(record),
        }, () => '部署'),
        h(Button, {
          size: 'small',
          onClick: () => handleEdit(record),
        }, () => '编辑'),
      ];
    },
  },
];

// 处理部署
const handleDeploy = (record: any) => {
  message.loading(`正在部署 ${record.name}...`, 2.5);
  configFilesApi.deployConfigFile(record.id)
    .then(() => {
      message.success(`${record.name} 部署成功！`);
    })
    .catch((error) => {
      console.error('部署失败:', error);
      message.error(`${record.name} 部署失败！`);
    });
};

// 处理编辑
const handleEdit = (record: any) => {
  currentFile.value = { ...record };
  modalVisible.value = true;
};

// 处理删除
const handleDelete = (record: any) => {
  message.loading(`正在删除 ${record.name}...`, 2.5);
  configFilesApi.deleteConfigFile(record.id)
    .then(() => {
      message.success(`${record.name} 删除成功！`);
      // 从列表中移除
      configFilesList.value = configFilesList.value.filter(item => item.id !== record.id);
    })
    .catch((error) => {
      console.error('删除失败:', error);
      message.error(`${record.name} 删除失败！`);
    });
};

// 处理添加
const handleAdd = () => {
  currentFile.value = {
    id: '',
    name: '',
    type: '配置文件',
    path: '/home/user/.config',
    description: '',
    lastModified: new Date().toISOString().split('T')[0],
  };
  modalVisible.value = true;
};

// 处理保存
const handleSave = () => {
  if (!currentFile.value.name) {
    message.warning('请输入配置文件名称');
    return;
  }

  const isNew = !currentFile.value.id;
  const apiCall = isNew 
    ? configFilesApi.addConfigFile(currentFile.value)
    : configFilesApi.updateConfigFile(currentFile.value.id, currentFile.value);

  message.loading(`正在${isNew ? '添加' : '更新'}配置文件...`, 2.5);
  apiCall
    .then(() => {
      message.success(`配置文件${isNew ? '添加' : '更新'}成功！`);
      modalVisible.value = false;
      // 重新获取列表
      fetchConfigFiles();
    })
    .catch((error) => {
      console.error(`${isNew ? '添加' : '更新'}失败:`, error);
      message.error(`配置文件${isNew ? '添加' : '更新'}失败！`);
    });
};

// 处理搜索
const handleSearch = () => {
  // 本地搜索实现，实际项目中可以调用后端API
  if (!searchValue.value) {
    return;
  }
};

// 表格行选择配置
const rowSelection = {
  selectedRowKeys: selectedRowKeys.value,
  onChange: (keys: string[]) => {
    selectedRowKeys.value = keys;
  },
};

// 批量部署
const handleBatchDeploy = () => {
  if (selectedRowKeys.value.length === 0) {
    message.warning('请先选择要部署的配置文件');
    return;
  }
  
  deployLoading.value = true;
  
  // 实际应用中这里会调用API进行批量部署
  setTimeout(() => {
    message.success(`已成功部署 ${selectedRowKeys.value.length} 个配置文件`);
    deployLoading.value = false;
    selectedRowKeys.value = [];
  }, 3000);
};

// 初始化
onMounted(() => {
  fetchConfigFiles();
});
</script>

<template>
  <div class="config-files-container">
    <a-row :gutter="16" class="page-header">
      <a-col :span="24">
        <a-page-header
          title="配置文件管理"
          subtitle="管理和部署您的个人配置文件"
        />
      </a-col>
    </a-row>
    
    <a-row :gutter="16" class="search-bar">
      <a-col :xs="24" :sm="16" :md="18">
        <a-input-search
          v-model:value="searchValue"
          placeholder="搜索配置文件名称、描述或路径"
          enter-button
        />
      </a-col>
      <a-col :xs="24" :sm="8" :md="6" class="action-buttons">
        <a-button
          type="primary"
          :loading="deployLoading"
          :disabled="selectedRowKeys.length === 0"
          style="margin-right: 8px"
          @click="handleBatchDeploy"
        >
          <template #icon><SaveOutlined /></template>
          批量部署 ({{ selectedRowKeys.length }})
        </a-button>
        <a-button type="primary" @click="handleAdd">
          <template #icon><FileAddOutlined /></template>
          添加
        </a-button>
      </a-col>
    </a-row>
    
    <a-row :gutter="16" class="content-area">
      <a-col :span="24">
        <a-table
          :columns="columns"
          :data-source="filteredConfigFilesList"
          :row-selection="rowSelection"
          :loading="loading"
          row-key="id"
        >
          <template #expandedRowRender="{ record }">
            <p style="margin: 0">{{ record.description }}</p>
          </template>
        </a-table>
      </a-col>
    </a-row>
    
    <!-- 配置文件编辑对话框 -->
    <a-modal
      v-model:visible="modalVisible"
      :title="currentFile?.id ? '编辑配置文件' : '添加配置文件'"
      @ok="handleSave"
    >
      <a-form layout="vertical">
        <a-form-item label="名称" required>
          <a-input v-model:value="currentFile.name" placeholder="配置文件名称" />
        </a-form-item>
        <a-form-item label="路径" required>
          <a-input-group compact>
            <a-input
              v-model:value="currentFile.path"
              style="width: calc(100% - 50px)"
              placeholder="配置文件路径"
            />
            <a-button>
              <template #icon><FolderOpenOutlined /></template>
            </a-button>
          </a-input-group>
        </a-form-item>
        <a-form-item label="描述">
          <a-textarea
            v-model:value="currentFile.description"
            placeholder="配置文件描述"
            :rows="3"
          />
        </a-form-item>
        <a-form-item label="类型">
          <a-select v-model:value="currentFile.type">
            <a-select-option value="editor">编辑器</a-select-option>
            <a-select-option value="dev">开发工具</a-select-option>
            <a-select-option value="terminal">终端</a-select-option>
            <a-select-option value="browser">浏览器</a-select-option>
          </a-select>
        </a-form-item>
      </a-form>
      <template #footer>
        <a-button key="back" @click="modalVisible = false">取消</a-button>
        <a-button
          v-if="currentFile?.id"
          key="delete"
          danger
          @click="handleDelete(currentFile)"
        >
          <template #icon><DeleteOutlined /></template>
          删除
        </a-button>
        <a-button key="submit" type="primary" @click="handleSave">
          <template #icon><SaveOutlined /></template>
          保存
        </a-button>
      </template>
    </a-modal>
  </div>
</template>

<style scoped>
.config-files-container {
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

@media (max-width: 576px) {
  .action-buttons {
    margin-top: 16px;
    justify-content: flex-start;
  }
}
</style>
