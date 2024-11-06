import { Table, TableContainer } from "@chakra-ui/react";

export default function ITable({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <TableContainer
      overflowY={'auto'}
      maxH={'60dvh'}
      bg={'white'}
      rounded={'xl'}
    >
      <Table variant={'simple'}>
        {children}
      </Table>
    </TableContainer>
  )
}