<template>
  <!-- <n-data-table
    :columns="columns"
    :data="data"
    :max-height="250"
    :scroll-x="1800"
    virtual-scroll
  /> -->
</template>

<script setup lang="ts">
import { h, reactive, ref } from 'vue'
import { NDataTable } from 'naive-ui'
import type { DataTableColumns } from 'naive-ui'

type RowData = {
  key: number
  name: string
  age: number
  address: string
}

const columns: DataTableColumns<RowData> = [
  {
    type: 'selection',
    fixed: 'left',
  },
  {
    title: 'Name',
    key: 'name',
    width: 200,
    fixed: 'left',
  },
  {
    title: 'Age',
    key: 'age',
    width: 100,
    fixed: 'left',
  },
  {
    title: 'Row',
    key: 'row',
    render(row, index) {
      return h('span', ['row ', index])
    },
  },
  {
    title: 'Row1',
    key: 'row1',
    render(row, index) {
      return h('span', ['row ', index])
    },
  },
  {
    title: 'Row2',
    key: 'row2',
    render(row, index) {
      return h('span', ['row ', index])
    },
    width: 100,
    fixed: 'right',
  },
  {
    title: 'Address',
    key: 'address',
    width: 200,
    fixed: 'right',
  },
]

// const data = reactive({
//   key: 1,
//   name: 'string',
//   age: 1,
//   address: 'string',
// }) as RowData[]
</script>

<style scoped></style>
