import type { Metadata } from 'next'

import '@/app/index.css'

export const metadata: Metadata = {
  title: 'Daifugo',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
