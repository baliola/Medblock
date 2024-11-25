"use client"

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Tag, Td, Tr } from "@chakra-ui/react";

import ITable from "@/components/table";
import ITableHeader from "@/components/table/header";
import ITableBody from "@/components/table/body";

import { uamTableHeader } from "@/constants/contents/uam/table";
import { Verification } from "@/libs/api/uam";

interface UAMTableProps {
  datas: {
    fullName: string;
    nikHash: string;
    verification: Verification;
  }[]
};

const TagStatus = ({ status }: { status: Verification }) => {
  switch (status) {
    case "pending":
      return <Tag colorScheme={'yellow'}>Pending</Tag>;
    case "accepted":
      return <Tag colorScheme={'green'}>Accepted</Tag>;
    case "rejected":
      return <Tag colorScheme={'red'}>Rejected</Tag>;
  }
}

export default function UAMTable({ datas }: UAMTableProps) {
  const router = useRouter();
  const params = useSearchParams();
  const pathname = usePathname();

  const onDetail = (id: string) => {
    const param = new URLSearchParams(params);
    param.set('user', id);

    const path = `${pathname}?${param.toString()}`;
    router.push(path);
  }

  return (
    <ITable>
      <ITableHeader headers={uamTableHeader} />
      <ITableBody>
        {datas?.map((data, index) => (
          <Tr key={index}
            cursor={'pointer'}
            _hover={{ bg: 'primary.100' }}
            onClick={() => onDetail(String(data.nikHash))}
          >
            <Td>{index + 1}</Td>
            <Td>{data.fullName}</Td>
            <Td>{data.nikHash}</Td>
            <Td>
              <TagStatus status={data.verification} />
            </Td>
          </Tr>
        ))}
      </ITableBody>
    </ITable>
  )
}