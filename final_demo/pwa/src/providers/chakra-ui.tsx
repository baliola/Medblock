'use client'

import { ChakraProvider, extendTheme } from '@chakra-ui/react'


const themes = extendTheme({
  colors: {
    primary: {
      950: "#090B2A",
      900: "#0C0F38",
      800: "#151A62",
      700: "#242DA8",
      600: "#3E48D6",
      500: "#7178E1",
      400: "#9CA2EA",
      300: "#C0C3F2",
      200: "#DBDDF7",
      100: "#EFF0FC",
      50: "#FBFBFE",
    },
    accent: {
      950: "#31020D",
      900: "#430212",
      800: "#790420",
      700: "#D40837",
      600: "#F72859",
      500: "#F96185",
      400: "#FB91AA",
      300: "#FCB9C9",
      200: "#FED8E0",
      100: "#FEEDF1",
      50: "#FFFBFC",
    },
    success: {
      950: "#0A2A09",
      900: "#0D340B",
      800: "#145010",
      700: "#1F7F1A",
      600: "#30C428",
      500: "#5FDD58",
      400: "#90E78B",
      300: "#B8F0B5",
      200: "#D7F6D5",
      100: "#EDFBEC",
      50: "#FBFEFA",
    },
    info: {
      950: "#012632",
      900: "#02303E",
      800: "#034C62",
      700: "#047B9F",
      600: "#06B8EE",
      500: "#41CFFA",
      400: "#7BDEFC",
      300: "#AAEAFD",
      200: "#CFF3FE",
      100: "#EAFAFE",
      50: "#FAFEFF",
    },
    warning: {
      950: "#321A01",
      900: "#472501",
      800: "#854602",
      700: "#EC7C04",
      600: "#FC9B33",
      500: "#FDB569",
      400: "#FDCC97",
      300: "#FEDEBC",
      200: "#FEEDDA",
      100: "#FFF7EE",
      50: "#FFFDFB",
    },
    danger: {
      950: "#2E0505",
      900: "#390606",
      800: "#590909",
      700: "#8F0F0F",
      600: "#DA1717",
      500: "#EC4C4C",
      400: "#F28383",
      300: "#F7AFAF",
      200: "#FAD2D2",
      100: "#FDEBEB",
      50: "#FEFAFA",
    },
    neutral: {
      950: "#000000",
      900: "#050505",
      800: "#131313",
      700: "#2B2B2B",
      600: "#5D5D5D",
      500: "#888888",
      400: "#ACACAC",
      300: "#CACACA",
      200: "#E1E1E1",
      100: "#F2F2F2",
      50: "#FCFCFC",
    },
  }
})

export function ChakraUIProvider({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <ChakraProvider theme={themes}>
      {children}
    </ChakraProvider>
  )
}