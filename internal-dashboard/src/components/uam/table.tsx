"use client"

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Tag, Td, Tr } from "@chakra-ui/react";

import ITable from "@/components/table";
import ITableHeader from "@/components/table/header";
import ITableBody from "@/components/table/body";

import { uamTableHeader } from "@/constants/contents/uam/table";
import { Verification } from "@/libs/api/uam";
import { PatientWithNik } from "@/canister/declarations/patient_registry/patient_registry.did";

interface UAMTableProps {
  datas: PatientWithNik[]
  filter: (datas: Array<PatientWithNik>) => PatientWithNik[]
};

const TagStatus = ({ status }: { status: Verification }) => {
  switch (status.toLowerCase()) {
    case "pending":
      return <Tag colorScheme={'yellow'}>Pending</Tag>;
    case "approved":
      return <Tag colorScheme={'green'}>Approved</Tag>;
    case "denied":
      return <Tag colorScheme={'red'}>Denied</Tag>;
  }
}

export default function UAMTable({ filter, datas }: UAMTableProps) {
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
        {filter(datas)?.map((data, index) => (
          <Tr key={index}
            cursor={'pointer'}
            _hover={{ bg: 'primary.100' }}
            onClick={() => onDetail(String(data.nik))}
          >
            <Td>{index + 1}</Td>
            <Td textTransform={"capitalize"}>{data.info.V1.name}</Td>
            <Td>{data.nik}</Td>
            <Td>
              <TagStatus status={Object.keys(data.info.V1.kyc_status)[0] as Verification} />
            </Td>
          </Tr>
        ))}
      </ITableBody>
    </ITable>
  )
}