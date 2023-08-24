import './globals.css'
import { ThemeProvider } from '@/components/theme-provider'
import { Inter } from 'next/font/google'
import { PropsWithChildren } from 'react'

const inter = Inter({ subsets: ['latin'] })

export default function RootLayout({ children }: PropsWithChildren) {
  return (
    <html lang='en'>
      <body className={inter.className}>
        <ThemeProvider attribute='class' defaultTheme='system' enableSystem>
          {children}
        </ThemeProvider>
      </body>
    </html>
  )
}
