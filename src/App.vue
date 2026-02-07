<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { marked } from "marked";
import "github-markdown-css/github-markdown.css";

// --- 基础状态 ---
const noteTitle = ref("");
const noteContent = ref("");
const isEditing = ref(true);
const statusMsg = ref("");
const textareaRef = ref<HTMLTextAreaElement | null>(null);
const scrollContainerRef = ref<HTMLDivElement | null>(null);
const allNotes = ref<string[]>([]);
const searchQuery = ref("");
const isSidebarOpen = ref(false);
const isSettingsOpen = ref(false);

// --- 用户设置状态 ---
const config = ref({
  savePath: "",
  fontSize: 16,
  fontColor: "#333333"
});

// --- 计算属性 ---
const filteredNotes = computed(() => {
  return allNotes.value.filter(name =>
    name.toLowerCase().includes(searchQuery.value.toLowerCase())
  );
});

const lineCount = computed(() => {
  return noteContent.value.split("\n").length;
});

const renderedHTML = computed(() => {
  return marked.parse(noteContent.value || "_开始写作..._");
});

const dynamicStyle = computed(() => ({
  fontSize: config.value.fontSize + 'px',
  color: config.value.fontColor
}));


watch(config, (newCfg) => {
  localStorage.setItem("editor_config", JSON.stringify(newCfg));
}, { deep: true });

async function selectFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: config.value.savePath
  });
  if (selected) {
    config.value.savePath = selected as string;
    refreshList();
  }
}

async function refreshList() {
  if (!config.value.savePath) return;
  try {
    allNotes.value = await invoke("list_notes", { path: config.value.savePath });
  } catch (err) {
    statusMsg.value = "无法读取目录";
  }
}

function createNewNote() {
  noteTitle.value = "";
  noteContent.value = "";
  isEditing.value = true;
  isSidebarOpen.value = false;
  localStorage.removeItem("last_note_title");
  nextTick(() => textareaRef.value?.focus());
}

async function loadNote(title: string) {
  try {
    const content: string = await invoke("read_note", { 
      path: config.value.savePath, 
      title 
    });
    noteTitle.value = title;
    noteContent.value = content;
    isEditing.value = false;
    isSidebarOpen.value = false;
    // 记忆当前打开的文件名
    localStorage.setItem("last_note_title", title);
    // 加载了正式文件，清除之前的草稿
    localStorage.removeItem("unsaved_draft"); 
    if (scrollContainerRef.value) scrollContainerRef.value.scrollTop = 0;
  } catch (err) {
    statusMsg.value = "读取失败";
  }
}

async function startEdit() {
  isEditing.value = true;
  await nextTick();
  textareaRef.value?.focus();
}

async function handleSave(showMsg = true) {
  if (!noteTitle.value.trim() || !config.value.savePath) {
    if (showMsg) statusMsg.value = "请确保有标题和路径";
    return;
  }
  try {
    await invoke("save_note", {
      path: config.value.savePath,
      title: noteTitle.value,
      content: noteContent.value,
    });
    if (showMsg) {
      statusMsg.value = "已保存";
      setTimeout(() => (statusMsg.value = ""), 2000);
    }
    refreshList();
    localStorage.setItem("last_note_title", noteTitle.value);
    localStorage.removeItem("unsaved_draft"); 
  } catch (err) {
    if (showMsg) statusMsg.value = "保存失败";
  }
}

// 2. 自动保存与防掉电逻辑
let autoSaveTimer: number | null = null;
watch([noteContent, noteTitle], () => {
  if (isEditing.value) {
    if (autoSaveTimer) clearTimeout(autoSaveTimer);
    
    autoSaveTimer = window.setTimeout(() => {
      if (noteTitle.value.trim()) {
        //有标题，直接静默保存到硬盘
        handleSave(false);
        statusMsg.value = "自动保存";
      } else if (noteContent.value.trim()) {
        // 没标题但有内容，存入草稿箱(localStorage)防止关机丢失
        localStorage.setItem("unsaved_draft", noteContent.value);
        statusMsg.value = "草稿已暂存";
      }
      setTimeout(() => { 
        if (statusMsg.value === '自动保存' || statusMsg.value === '草稿已暂存') statusMsg.value = "" 
      }, 1000);
    }, 800);
  }
});

// 3. 初始化加载
onMounted(async () => {
  try {
    // 恢复用户设置
    const savedConfig = localStorage.getItem("editor_config");
    if (savedConfig) {
      config.value = JSON.parse(savedConfig);
    } else {
      const defaultPath: string = await invoke("get_default_save_path");
      config.value.savePath = defaultPath;
    }

    //获取文件列表
    await refreshList();
    
    //恢复上次会话
    const lastTitle = localStorage.getItem("last_note_title");
    const draft = localStorage.getItem("unsaved_draft");

    if (lastTitle && allNotes.value.includes(lastTitle)) {
      loadNote(lastTitle);
    } else if (draft) {
      noteContent.value = draft;
      statusMsg.value = "已恢复未保存草稿";
      setTimeout(() => (statusMsg.value = ""), 3000);
    }
  } catch (err) {
    statusMsg.value = "初始化失败";
  }
});
</script>

<template>
  <div class="app-layout">
    <aside class="sidebar" :class="{ 'sidebar-open': isSidebarOpen }">
      <div class="sidebar-header" v-if="isSidebarOpen">
        <input v-model="searchQuery" placeholder="搜索笔记..." class="search-input" />
      </div>
      <div class="note-list" v-if="isSidebarOpen">
        <div 
          v-for="name in filteredNotes" 
          :key="name" 
          class="note-item"
          :class="{ active: noteTitle === name }"
          @click="loadNote(name)"
        >
          {{ name }}
        </div>
      </div>
    </aside>

    <main class="main-content">
      <header class="app-header">
        <div class="nav-group">
          <button class="icon-btn" @click="isSidebarOpen = !isSidebarOpen">
            <svg viewBox="0 0 24 24" width="20" height="20"><path fill="currentColor" d="M3,6H21V8H3V6M3,11H21V13H3V11M3,16H21V18H3V16Z" /></svg>
          </button>
          <button class="icon-btn" @click="createNewNote">
            <svg viewBox="0 0 24 24" width="20" height="20"><path fill="currentColor" d="M19,13H13V19H11V13H5V11H11V5H13V11H19V13Z" /></svg>
          </button>
          <button class="icon-btn" @click="isSettingsOpen = true">
            <svg viewBox="0 0 24 24" width="20" height="20"><path fill="currentColor" d="M12,15.5A3.5,3.5 0 0,1 8.5,12A3.5,3.5 0 0,1 12,8.5A3.5,3.5 0 0,1 15.5,12A3.5,3.5 0 0,1 12,15.5M19.43,12.97C19.47,12.65 19.5,12.33 19.5,12C19.5,11.67 19.47,11.34 19.43,11L21.54,9.37C21.73,9.22 21.78,8.95 21.66,8.73L19.66,5.27C19.54,5.05 19.27,4.96 19.05,5.05L16.56,6.05C16.04,5.66 15.47,5.32 14.87,5.07L14.49,2.42C14.46,2.18 14.25,2 14,2H10C9.75,2 9.54,2.18 9.51,2.42L9.13,5.07C8.53,5.32 7.96,5.66 7.44,6.05L4.95,5.05C4.73,4.96 4.46,5.05 4.34,5.27L2.34,8.73C2.22,8.95 2.27,9.22 2.46,9.37L4.57,11C4.53,11.34 4.5,11.67 4.5,12C4.5,12.33 4.53,12.65 4.57,12.97L2.46,14.63C2.27,14.78 2.22,15.05 2.34,15.27L4.34,18.73C4.46,18.95 4.73,19.04 4.95,18.95L7.44,17.94C7.96,18.34 8.53,18.68 9.13,18.93L9.51,21.58C9.54,21.82 9.75,22 10,22H14C14.25,22 14.46,21.82 14.49,21.58L14.87,18.93C15.47,18.68 16.04,18.34 16.56,17.94L19.05,18.95C19.27,19.04 19.54,18.95 19.66,18.73L21.66,15.27C21.78,15.05 21.73,14.78 21.54,14.63L19.43,12.97Z" /></svg>
          </button>
        </div>

        <input class="title-input" v-model="noteTitle" placeholder="无标题笔记" />

        <div class="actions">
          <span class="status-hint">{{ statusMsg }}</span>
          <button class="mode-btn" @click="isEditing = !isEditing">{{ isEditing ? '预览' : '编辑' }}</button>
          <button class="save-btn" @click="() => handleSave()">保存</button>
        </div>
      </header>

      <div class="scroll-container" ref="scrollContainerRef">
        <div class="editor-wrapper" :style="dynamicStyle">
          <template v-if="isEditing">
            <div class="line-numbers">
              <div v-for="n in lineCount" :key="n" class="line-no">{{ n }}</div>
            </div>
            <textarea 
              ref="textareaRef"
              v-model="noteContent" 
              class="editor"
              placeholder="开始书写..."
              spellcheck="false"
            ></textarea>
          </template>
          
          <div 
            v-else 
            class="markdown-body preview-area" 
            @click="startEdit"
            v-html="renderedHTML"
          ></div>
        </div>
      </div>

      <div v-if="isSettingsOpen" class="modal-overlay" @click.self="isSettingsOpen = false">
        <div class="modal-content">
          <h3>界面设置</h3>
          <div class="setting-group">
            <label>保存位置</label>
            <div class="path-picker">
              <input type="text" :value="config.savePath" readonly />
              <button class="picker-btn" @click="selectFolder">选择</button>
            </div>
          </div>
          <div class="setting-group">
            <label>字体大小: {{ config.fontSize }}px</label>
            <input type="range" min="12" max="32" v-model="config.fontSize" />
          </div>
          <div class="setting-group">
            <label>正文颜色</label>
            <input type="color" v-model="config.fontColor" class="color-picker" />
          </div>
          <button class="close-btn" @click="isSettingsOpen = false">完成</button>
          <div class="version-info">版本 0.0.1</div>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
:global(html, body) {
  margin: 0;
  padding: 0;
  overflow: hidden !important;
  height: 100%;
}
:root {
  --bg-color: #ffffff;
  --sidebar-bg: #f8f9fa;
  --border-color: rgba(128, 128, 128, 0.1);
  --text-color: #333333;
  --line-no-color: #999;
}
@media (prefers-color-scheme: dark) {
  :root {
    --bg-color: #1e1e1e;
    --sidebar-bg: #252526;
    --border-color: #333333;
    --text-color: #cccccc;
  }
}
.app-layout {
  display: flex;
  height: 100vh;
  width: 100vw;
  background-color: var(--bg-color);
  color: var(--text-color);
  overflow: hidden; 
}
.sidebar {
  width: 0;
  height: 100%;
  background-color: var(--sidebar-bg);
  transition: width 0.3s ease;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  flex-shrink: 0;
}
.sidebar-open {
  width: 280px;
  border-right: 1px solid var(--border-color);
}
.sidebar-header {
  padding: 15px;
  border-bottom: 1px solid var(--border-color);
}
.search-input {
  width: 100%;
  padding: 8px 12px;
  border-radius: 6px;
  border: 1px solid var(--border-color);
  background: var(--bg-color);
  color: inherit;
  outline: none;
}
.note-list {
  flex: 1;
  overflow-y: auto;
}
.note-item {
  padding: 12px 20px;
  cursor: pointer;
  border-bottom: 1px solid var(--border-color);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-size: 0.9rem;
}
.note-item:hover { background: rgba(128, 128, 128, 0.05); }
.note-item.active {
  background-color: rgba(57, 108, 216, 0.1);
  border-left: 4px solid #396cd8;
}
.main-content {
  flex: 1;
  height: 100vh;
  overflow: hidden; 
  display: flex;
  flex-direction: column;
}
.app-header {
  height: 60px;
  display: flex;
  align-items: center;
  padding: 0 20px;
  border-bottom: 1px solid var(--border-color);
  gap: 15px;
  flex-shrink: 0;
  background: var(--bg-color);
}
.scroll-container {
  flex: 1;
  overflow-y: auto; 
  overflow-x: hidden;
  width: 100%;
}
.scroll-container::-webkit-scrollbar {
  width: 8px;
}
.scroll-container::-webkit-scrollbar-thumb {
  background: rgba(128,128,128,0.2);
  border-radius: 4px;
}
.editor-wrapper {
  display: flex;
  min-height: 100%;
  width: 100%;
  line-height: 1.5;
}
.line-numbers {
  padding: 40px 10px 40px 20px;
  color: var(--line-no-color);
  text-align: right;
  user-select: none;
  font-family: 'Consolas', 'Monaco', monospace;
  border-right: 1px solid var(--border-color);
  background: var(--bg-color);
  min-width: 50px;
  box-sizing: border-box;
  flex-shrink: 0;
}
.line-no {
  height: 1.5em;
}
.editor {
  flex: 1; 
  padding: 40px;
  border: none;
  resize: none;
  outline: none;
  background: transparent;
  font-family: inherit;
  font-size: inherit;
  color: inherit;
  line-height: 1.5;
  overflow: hidden;
  box-sizing: border-box;
  margin: 0;
  display: block;
  vertical-align: top;
  white-space: pre;
}
.preview-area {
  flex: 1;
  padding: 40px;
  background-color: transparent !important;
  color: inherit !important;
  line-height: 1.5;
}
.title-input {
  flex: 1;
  border: none;
  background: transparent;
  font-size: 1.2rem;
  font-weight: 700;
  outline: none;
  color: inherit;
}
.nav-group {
  display: flex;
  gap: 4px;
}
.icon-btn {
  background: transparent;
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 8px;
  border-radius: 6px;
  display: flex;
}
.icon-btn:hover { background: rgba(128, 128, 128, 0.1); }
.actions {
  display: flex;
  align-items: center;
  gap: 10px;
}
.mode-btn, .save-btn {
  padding: 6px 15px;
  border-radius: 20px;
  cursor: pointer;
  font-size: 0.85rem;
}
.save-btn { background: #396cd8; color: white; border: none; }
.mode-btn { background: transparent; color: var(--text-color); border: 1px solid var(--border-color); }
.modal-overlay {
  position: fixed;
  top: 0; left: 0; right: 0; bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
  backdrop-filter: blur(4px);
}
.modal-content {
  background-color: #ffffff; 
  color: #333333;
  padding: 30px;
  border-radius: 16px;
  width: 400px;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(0, 0, 0, 0.1);
}
@media (prefers-color-scheme: dark) {
  .modal-content {
    background-color: #252526 !important;
    color: #cccccc !important;
    border: 1px solid #444444;
  }
}
.setting-group {
  margin-bottom: 25px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.setting-group label {
  font-weight: 600;
  font-size: 0.95rem;
}
.path-picker { display: flex; gap: 10px; }
.path-picker input { 
  flex: 1; 
  padding: 10px; 
  background: rgba(0, 0, 0, 0.05); 
  border: 1px solid rgba(0, 0, 0, 0.1); 
  color: inherit;
  border-radius: 8px;
}
@media (prefers-color-scheme: dark) {
  .path-picker input { background: rgba(255, 255, 255, 0.05); }
}
.picker-btn {
  padding: 0 15px;
  background: #f0f0f0;
  border: 1px solid #ccc;
  border-radius: 8px;
  cursor: pointer;
}
.color-picker { width: 100%; height: 45px; border: none; cursor: pointer; border-radius: 8px; background: transparent; }
.close-btn {
  width: 100%;
  padding: 14px;
  background: #396cd8;
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: bold;
}
.version-info {
  text-align: center;
  margin-top: 15px;
  font-size: 0.75rem;
  opacity: 0.4;
  letter-spacing: 1px;
}
.status-hint { font-size: 0.8rem; opacity: 0.7; }
</style>