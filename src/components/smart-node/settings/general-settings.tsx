import {
  Box,
  Stack,
  Switch,
  TextField,
  Typography,
} from '@mui/material'
import { useTranslation } from 'react-i18next'

import { BaseFieldset } from '@/components/base'
import {
  useSmartNodeConfig,
  useSmartNodeEnabled,
} from '@/hooks/use-smart-node'

export const GeneralSettings = () => {
  const { t } = useTranslation('smartNode')
  const { enabled, toggle, toggling } =
    useSmartNodeEnabled()
  const { config, patchConfig, patching } =
    useSmartNodeConfig()

  const handleToggle = async () => {
    await toggle(!enabled)
  }

  return (
    <BaseFieldset title={t('settings.general.title')}>
      <Stack spacing={2}>
        <Stack
          direction="row"
          alignItems="center"
          justifyContent="space-between"
        >
          <Typography variant="body2">
            {t('settings.general.enable')}
          </Typography>
          <Switch
            checked={enabled}
            onChange={handleToggle}
            disabled={toggling}
          />
        </Stack>

        <Box
          sx={{
            display: 'grid',
            gridTemplateColumns: '1fr 1fr',
            gap: 2,
          }}
        >
          <TextField
            size="small"
            label={t('settings.general.retentionDays')}
            type="number"
            value={config?.history.retention_days ?? 30}
            onChange={(e) =>
              patchConfig({
                history: {
                  ...config!.history,
                  retention_days: Number(e.target.value),
                },
              })
            }
            disabled={patching}
          />

          <TextField
            size="small"
            label={t('settings.general.cleanupTime')}
            type="time"
            value={config?.history.cleanup_time ?? '03:00'}
            onChange={(e) =>
              patchConfig({
                history: {
                  ...config!.history,
                  cleanup_time: e.target.value,
                },
              })
            }
            disabled={patching}
            InputLabelProps={{ shrink: true }}
          />

          <TextField
            size="small"
            label={t(
              'settings.general.healthCheckInterval',
            )}
            type="number"
            value={
              config?.health_check.interval_minutes ?? 5
            }
            onChange={(e) =>
              patchConfig({
                health_check: {
                  ...config!.health_check,
                  interval_minutes: Number(e.target.value),
                },
              })
            }
            disabled={patching}
          />

          <TextField
            size="small"
            label={t(
              'settings.general.healthCheckTimeout',
            )}
            type="number"
            value={config?.health_check.timeout_ms ?? 5000}
            onChange={(e) =>
              patchConfig({
                health_check: {
                  ...config!.health_check,
                  timeout_ms: Number(e.target.value),
                },
              })
            }
            disabled={patching}
          />
        </Box>

        <Stack
          direction="row"
          alignItems="center"
          justifyContent="space-between"
        >
          <Typography variant="body2">
            {t('settings.general.autoCleanup')}
          </Typography>
          <Switch
            checked={config?.history.auto_cleanup ?? true}
            onChange={(e) =>
              patchConfig({
                history: {
                  ...config!.history,
                  auto_cleanup: e.target.checked,
                },
              })
            }
            disabled={patching}
          />
        </Stack>
      </Stack>
    </BaseFieldset>
  )
}
