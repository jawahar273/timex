import Image from 'next/image'

import { Date } from '@timex/components'

import { Inter as FontSans } from "next/font/google"
 
import { cn } from "@timex/utils"
 
export const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
})
 

export default function Home() {
  return (
<main>
<Date />
       </main>
  )
}
