import { Button, TextField } from '@mui/material'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { BaseDialog } from '@/components/base'

interface ImportDialogProps {
  open: boolean
  onClose: () => void
  onImport: (content: string, format: string) => void
  importing: boolean
}

export const ImportDialog = ({
  open,
  onClose,
  onImport,
  importing,
}: ImportDialogProps) => {
  const { t } = useTranslation('smartNode')
  const [content, setContent] = useState('')
  const [format, setFormat] = useState('uri')

  const handleImport = () => {
    if (!content.trim()) return
    onImport(content.trim(), format)
    setContent('')
  }

  return (
    <BaseDialog
      title={t('pool.importDialog.title')}
      open={open}
      onClose={onClose}
      onCancel={onClose}
      onOk={handleImport}
      okBtn={t('pool.importDialog.confirm')}
      loading={importing}
      disableOk={!content.trim()}
      contentSx={{ minWidth: 400, display: 'flex', flexDirection: 'column', gap: 2 }}
    >
      <TextField
        select
        fullWidth
        size="small"
        label={t('pool.importDialog.format')}
        value={format}
        onChange={(e) => setFormat(e.target.value)}
        SelectProps={{
          native: true,
        }}
      >
        <option value="uri">URI</option>
        <option value="clash">Clash YAML</option>
        <option value="base64">Base64</option>
      </TextField>
      <TextField
        fullWidth
        multiline
        rows={8}
        label={t('pool.importDialog.content')}
        value={content}
        onChange={(e) => setContent(e.target.value)}
        placeholder={t('pool.importDialog.placeholder')}
      />
    </BaseDialog>
  )
}
