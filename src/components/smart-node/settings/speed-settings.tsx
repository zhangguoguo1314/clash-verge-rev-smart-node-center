import {
  Box,
  Slider,
  Stack,
  Switch,
  TextField,
  Typography,
} from '@mui/material'
import { useTranslation } from 'react-i18next'

import { BaseFieldset } from '@/components/base'
import { useSmartNodeConfig } from '@/hooks/use-smart-node'

export const SpeedSettings = () => {
  const { t } = useTranslation('smartNode')
  const { config, patchConfig, patching } =
    useSmartNodeConfig()

  const speedTest = config?.speed_test
  const scoring = config?.scoring
  const autoSwitch = config?.auto_switch

  return (
    <Stack spacing={2}>
      <BaseFieldset title={t('settings.speed.title')}>
        <Stack spacing={2}>
          <Box
            sx={{
              display: 'grid',
              gridTemplateColumns: '1fr 1fr',
              gap: 2,
            }}
          >
            <TextField
              size="small"
              label={t('settings.speed.concurrentLimit')}
              type="number"
              value={speedTest?.concurrent_limit ?? 8}
              onChange={(e) =>
                patchConfig({
                  speed_test: {
                    ...speedTest!,
                    concurrent_limit: Number(e.target.value),
                  },
                })
              }
              disabled={patching}
            />
            <TextField
              size="small"
              label={t('settings.speed.tcpTimeout')}
              type="number"
              value={speedTest?.tcp_timeout_ms ?? 3000}
              onChange={(e) =>
                patchConfig({
                  speed_test: {
                    ...speedTest!,
                    tcp_timeout_ms: Number(e.target.value),
                  },
                })
              }
              disabled={patching}
            />
            <TextField
              size="small"
              label={t('settings.speed.httpTimeout')}
              type="number"
              value={speedTest?.http_timeout_ms ?? 5000}
              onChange={(e) =>
                patchConfig({
                  speed_test: {
                    ...speedTest!,
                    http_timeout_ms: Number(e.target.value),
                  },
                })
              }
              disabled={patching}
            />
            <TextField
              size="small"
              label={t('settings.speed.downloadTimeout')}
              type="number"
              value={
                speedTest?.download_timeout_ms ?? 10000
              }
              onChange={(e) =>
                patchConfig({
                  speed_test: {
                    ...speedTest!,
                    download_timeout_ms: Number(
                      e.target.value,
                    ),
                  },
                })
              }
              disabled={patching}
            />
          </Box>

          <TextField
            size="small"
            fullWidth
            label={t('settings.speed.downloadUrl')}
            value={speedTest?.download_url ?? ''}
            onChange={(e) =>
              patchConfig({
                speed_test: {
                  ...speedTest!,
                  download_url: e.target.value,
                },
              })
            }
            disabled={patching}
          />

          <Typography variant="body2">
            {t('settings.speed.downloadSize')}:{' '}
            {speedTest?.download_size ?? 10} MB
          </Typography>
          <Slider
            value={speedTest?.download_size ?? 10}
            onChange={(_, v) =>
              patchConfig({
                speed_test: {
                  ...speedTest!,
                  download_size: v as number,
                },
              })
            }
            min={1}
            max={100}
            step={1}
            disabled={patching}
          />
        </Stack>
      </BaseFieldset>

      <BaseFieldset title={t('settings.scoring.title')}>
        <Stack spacing={1.5}>
          <Typography variant="body2">
            {t('settings.scoring.latencyWeight')}:{' '}
            {((scoring?.latency_weight ?? 30) * 100).toFixed(0)}%
          </Typography>
          <Slider
            value={(scoring?.latency_weight ?? 0.3) * 100}
            onChange={(_, v) =>
              patchConfig({
                scoring: {
                  ...scoring!,
                  latency_weight: (v as number) / 100,
                },
              })
            }
            min={0}
            max={100}
            disabled={patching}
          />

          <Typography variant="body2">
            {t('settings.scoring.speedWeight')}:{' '}
            {((scoring?.speed_weight ?? 40) * 100).toFixed(0)}%
          </Typography>
          <Slider
            value={(scoring?.speed_weight ?? 0.4) * 100}
            onChange={(_, v) =>
              patchConfig({
                scoring: {
                  ...scoring!,
                  speed_weight: (v as number) / 100,
                },
              })
            }
            min={0}
            max={100}
            disabled={patching}
          />

          <Typography variant="body2">
            {t('settings.scoring.stabilityWeight')}:{' '}
            {((scoring?.stability_weight ?? 20) * 100).toFixed(0)}%
          </Typography>
          <Slider
            value={(scoring?.stability_weight ?? 0.2) * 100}
            onChange={(_, v) =>
              patchConfig({
                scoring: {
                  ...scoring!,
                  stability_weight: (v as number) / 100,
                },
              })
            }
            min={0}
            max={100}
            disabled={patching}
          />
        </Stack>
      </BaseFieldset>

      <BaseFieldset title={t('settings.autoSwitch.title')}>
        <Stack spacing={2}>
          <Stack
            direction="row"
            alignItems="center"
            justifyContent="space-between"
          >
            <Typography variant="body2">
              {t('settings.autoSwitch.enable')}
            </Typography>
            <Switch
              checked={autoSwitch?.enabled ?? false}
              onChange={(e) =>
                patchConfig({
                  auto_switch: {
                    ...autoSwitch!,
                    enabled: e.target.checked,
                  },
                })
              }
              disabled={patching}
            />
          </Stack>

          <TextField
            size="small"
            fullWidth
            label={t('settings.autoSwitch.strategy')}
            value={autoSwitch?.default_strategy ?? 'fastest'}
            onChange={(e) =>
              patchConfig({
                auto_switch: {
                  ...autoSwitch!,
                  default_strategy: e.target.value,
                },
              })
            }
            select
            SelectProps={{ native: true }}
            disabled={patching}
          >
            <option value="fastest">Fastest</option>
            <option value="most_stable">Most Stable</option>
            <option value="sequential">Sequential</option>
            <option value="random">Random</option>
            <option value="weighted">Weighted</option>
            <option value="ai_recommend">AI Recommend</option>
          </TextField>

          <TextField
            size="small"
            label={t('settings.autoSwitch.interval')}
            type="number"
            value={
              autoSwitch?.default_interval_minutes ?? 30
            }
            onChange={(e) =>
              patchConfig({
                auto_switch: {
                  ...autoSwitch!,
                  default_interval_minutes: Number(
                    e.target.value,
                  ),
                },
              })
            }
            disabled={patching}
          />

          <TextField
            size="small"
            label={t(
              'settings.autoSwitch.failThreshold',
            )}
            type="number"
            value={autoSwitch?.fail_count_threshold ?? 3}
            onChange={(e) =>
              patchConfig({
                auto_switch: {
                  ...autoSwitch!,
                  fail_count_threshold: Number(
                    e.target.value,
                  ),
                },
              })
            }
            disabled={patching}
          />
        </Stack>
      </BaseFieldset>
    </Stack>
  )
}
