import { Thead, Tr } from "@chakra-ui/react"
import { TH } from "@/components/table"

export default function ITableHeader({
  headers
}: {
  headers: string[]
}) {
  return (
    <Thead pos={'sticky'} top={0} bg={'white'} zIndex={999}>
      <Tr>
        {headers.map((header, index) => (
          <TH key={index} fontWeight={'bold'}>
            {header}
          </TH>
        ))}
      </Tr>
    </Thead>
  )
}