import { Td, Thead, Tr } from "@chakra-ui/react"

export default function ITableHeader({
  headers
}: {
  headers: string[]
}) {
  return (
    <Thead pos={'sticky'} top={0} bg={'white'} zIndex={999}>
      <Tr>
        {headers.map((header, index) => (
          <Td key={index} fontWeight={'bold'}>
            {header}
          </Td>
        ))}
      </Tr>
    </Thead>
  )
}