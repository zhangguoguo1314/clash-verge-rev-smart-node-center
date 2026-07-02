import {
  Button,
  Stack,
  Typography,
} from '@mui/material'
import { Add, Download, Upload } from '@mui/icons-material'
import { useMemo, useState } from 'react'
import { useTranslation } from 'react-i18next'

import { usePool } from '@/hooks/use-smart-node'
import {
  BaseSearchBox,
} from '@/components/base'
import { NodeItem, NodeList } from '@/components/smart-node/pool'

export const SmartNodePool = () => {
  const { t } = useTranslation('smartNode')
  const {
    pool,
    poolStats,
    importNodes,
    importing,
    removeNode,
    removing,
  } = usePool()

  const [search, setSearch] = useState('')
  const [importOpen, setImportOpen] = useState(false)
  const [selectedUid, setSelectedUid] = useState<
    string | null
  >(null)

  const filteredPool = useMemo(() => {
    if (!pool) return []
    if (!search) return pool
    const keyword = search.toLowerCase()
    return pool.filter(
      (node) =>
        node.name.toLowerCase().includes(keyword) ||
        node.address.toLowerCase().includes(keyword) ||
        node.protocol.toLowerCase().includes(keyword) ||
        node.tags.some((tag) =>
          tag.toLowerCase().includes(keyword),
        ),
    )
  }, [pool, search])

  const handleImport = async (
    content: string,
    format: string,
  ) => {
    await importNodes(content, format, 'manual')
    setImportOpen(false)
  }

  const handleDelete = async (uid: string) => {
    await removeNode(uid)
    if (selectedUid === uid) {
      setSelectedUid(null)
    }
  }

  return (
    <Stack spacing={2}>
      <Stack
        direction="row"
        alignItems="center"
        justifyContent="space-between"
      >
        <Typography variant="h6">
          {t('pool.title')}{' '}
          <Typography
            component="span"
            variant="body2"
            color="text.secondary"
          >
            {poolStats
              ? `${poolStats.healthy_count}/${poolStats.total_count}`
              : '-'}
          </Typography>
        </Typography>
        <Stack direction="row" spacing={1}>
          <Button
            size="small"
            variant="outlined"
            startIcon={<Upload />}
            onClick={() => setImportOpen(true)}
            disabled={importing}
          >
            {t('pool.import')}
          </Button>
          <Button
            size="small"
            variant="outlined"
            startIcon={<Download />}
          >
            {t('pool.export')}
          </Button>
          <Button
            size="small"
            variant="outlined"
            startIcon={<Add />}
          >
            {t('pool.add')}
          </Button>
        </Stack>
      </Stack>

      <BaseSearchBox
        value={search}
        onSearch={(v) => setSearch(v)}
        placeholder={t('pool.search')}
      />

      <NodeList
        nodes={filteredPool}
        selectedUid={selectedUid}
        onSelect={setSelectedUid}
      />

      {selectedUid && (
        <NodeItem
          uid={selectedUid}
          onDelete={handleDelete}
          deleting={removing}
        />
      )}
    </Stack>
  )
}
