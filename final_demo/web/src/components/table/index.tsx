import { Table, TableCellProps, TableContainer, Td, Th } from "@chakra-ui/react";

export const TD = ({ children, ...props }: TableCellProps) => {
  return (
    <Td
      fontSize={{ base: 'xs', xl: 'sm' }}
      {...props}
    >
      {children}
    </Td>
  )
}

export const TH = ({ children, ...props }: TableCellProps) => {
  return (
    <Th
      fontSize={{ base: 'xs' }}
      fontWeight={'bold'}
      color={'neutral.700'}
      {...props}
    >
      {children}
    </Th>
  )
}

export default function ITable({
  children
}: {
  children: React.ReactNode
}) {
  return (
    <TableContainer
      overflow={'auto'}
      maxH={'80dvh'}
      bg={'white'}
      rounded={'xl'}
    >
      <Table variant={'simple'}>
        {children}
      </Table>
    </TableContainer>
  )
}