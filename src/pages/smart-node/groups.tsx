import {
  Button,
  Stack,
  Typography,
} from '@mui/material'
import { Add, CheckCircle } from '@mui/icons-material'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { useGroups } from '@/hooks/use-smart-node'
import {
  GroupEditor,
  GroupList,
} from '@/components/smart-node/groups'

export const SmartNodeGroups = () => {
  const { t } = useTranslation('smartNode')
  const {
    groups,
    createGroup,
    updateGroup,
    deleteGroup,
    detectGroup,
    detecting,
  } = useGroups()

  const [editorOpen, setEditorOpen] = useState(false)
  const [editingGroup, setEditingGroup] = useState<
    string | null
  >(null)

  const handleCreate = () => {
    setEditingGroup(null)
    setEditorOpen(true)
  }

  const handleEdit = (groupId: string) => {
    setEditingGroup(groupId)
    setEditorOpen(true)
  }

  const handleDetect = async (groupId: string) => {
    await detectGroup(groupId)
  }

  const handleSave = async (
    group: Record<string, unknown>,
  ) => {
    if (editingGroup) {
      await updateGroup(editingGroup, group)
    } else {
      await createGroup(group)
    }
    setEditorOpen(false)
  }

  const handleDelete = async (groupId: string) => {
    await deleteGroup(groupId)
  }

  return (
    <Stack spacing={2}>
      <Stack
        direction="row"
        alignItems="center"
        justifyContent="space-between"
      >
        <Typography variant="h6">
          {t('groups.title')}
        </Typography>
        <Button
          size="small"
          variant="outlined"
          startIcon={<Add />}
          onClick={handleCreate}
        >
          {t('groups.create')}
        </Button>
      </Stack>

      <GroupList
        groups={groups}
        onEdit={handleEdit}
        onDelete={handleDelete}
        onDetect={handleDetect}
        detecting={detecting}
      />

      <GroupEditor
        open={editorOpen}
        onClose={() => setEditorOpen(false)}
        onSave={handleSave}
        group={
          editingGroup
            ? groups.find((g) => g.id === editingGroup)
            : undefined
        }
      />
    </Stack>
  )
}
