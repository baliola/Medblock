import { Tbody } from "@chakra-ui/react";

export default function ITableBody({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <Tbody maxH={'80dvh'}>
      {children}
    </Tbody>
  )
}