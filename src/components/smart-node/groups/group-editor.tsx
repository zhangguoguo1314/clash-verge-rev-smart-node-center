import {
  Box,
  Button,
  Chip,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  Stack,
  TextField,
  Typography,
} from '@mui/material'
import { Close } from '@mui/icons-material'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import type { SmartGroup } from '@/types/smart-node'
import { usePool } from '@/hooks/use-smart-node'

interface GroupEditorProps {
  open: boolean
  onClose: () => void
  onSave: (group: Record<string, unknown>) => Promise<void>
  group?: SmartGroup
}

export const GroupEditor = ({
  open,
  onClose,
  onSave,
  group,
}: GroupEditorProps) => {
  const { t } = useTranslation('smartNode')
  const { pool } = usePool()

  const [name, setName] = useState(group?.name ?? '')
  const [description, setDescription] = useState(
    group?.description ?? '',
  )
  const [selectedUids, setSelectedUids] = useState<Set<string>>(
    new Set(group?.node_uids ?? []),
  )
  const [autoSwitchEnabled, setAutoSwitchEnabled] = useState(
    group?.auto_switch.enabled ?? false,
  )
  const [strategy, setStrategy] = useState(
    group?.auto_switch.strategy ?? 'fastest',
  )
  const [interval, setIntervalVal] = useState(
    group?.auto_switch.interval_minutes ?? 30,
  )

  const handleToggleNode = (uid: string) => {
    setSelectedUids((prev) => {
      const next = new Set(prev)
      if (next.has(uid)) {
        next.delete(uid)
      } else {
        next.add(uid)
      }
      return next
    })
  }

  const handleSave = async () => {
    await onSave({
      name,
      description,
      node_uids: [...selectedUids],
      auto_switch: {
        enabled: autoSwitchEnabled,
        strategy,
        interval_minutes: interval,
      },
      detection: { urls: [] },
      auto_sync: false,
      sync_interval_minutes: 60,
      fallback_node_uids: [],
    })
  }

  return (
    <Dialog open={open} onClose={onClose} maxWidth="sm" fullWidth>
      <DialogTitle>{t('groups.editor.title')}</DialogTitle>
      <DialogContent>
        <Stack spacing={2}>
          <TextField
            fullWidth
            size="small"
            label={t('groups.editor.name')}
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <TextField
            fullWidth
            size="small"
            label={t('groups.editor.description')}
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            multiline
            rows={2}
          />

          <Typography variant="subtitle2">
            {t('groups.editor.selectNodes')}
          </Typography>
          <Box
            sx={{
              maxHeight: 200,
              overflow: 'auto',
              border: 1,
              borderColor: 'divider',
              borderRadius: 1,
              p: 1,
            }}
          >
            {(pool ?? []).map((node) => {
              const isSelected = selectedUids.has(node.uid)
              return (
                <Chip
                  key={node.uid}
                  label={node.name}
                  size="small"
                  variant={isSelected ? 'filled' : 'outlined'}
                  color={isSelected ? 'primary' : 'default'}
                  onClick={() => handleToggleNode(node.uid)}
                  sx={{ m: 0.5 }}
                  onDelete={
                    isSelected
                      ? () => handleToggleNode(node.uid)
                      : undefined
                  }
                  deleteIcon={
                    isSelected ? (
                      <Close fontSize="small" />
                    ) : undefined
                  }
                />
              )
            })}
          </Box>

          <Box>
            <Typography variant="subtitle2">
              {t('groups.editor.autoSwitch')}
            </Typography>
            <Stack direction="row" spacing={2} mt={1} alignItems="center">
              <Chip
                label={autoSwitchEnabled ? 'ON' : 'OFF'}
                color={autoSwitchEnabled ? 'success' : 'default'}
                onClick={() => setAutoSwitchEnabled((v) => !v)}
              />
              <TextField
                size="small"
                label={t('groups.editor.strategy')}
                value={strategy}
                onChange={(e) => setStrategy(e.target.value)}
                select
                SelectProps={{ native: true }}
                sx={{ minWidth: 120 }}
              >
                <option value="fastest">Fastest</option>
                <option value="most_stable">Most Stable</option>
                <option value="sequential">Sequential</option>
                <option value="random">Random</option>
              </TextField>
              <TextField
                size="small"
                label={t('groups.editor.interval')}
                value={interval}
                onChange={(e) =>
                  setIntervalVal(Number(e.target.value))
                }
                type="number"
                sx={{ minWidth: 80 }}
              />
            </Stack>
          </Box>
        </Stack>
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>
          {t('groups.editor.cancel')}
        </Button>
        <Button
          onClick={handleSave}
          variant="contained"
          disabled={!name.trim()}
        >
          {t('groups.editor.save')}
        </Button>
      </DialogActions>
    </Dialog>
  )
}
