"use client";

import { usePathname, useRouter, useSearchParams } from "next/navigation";
import { Td, Tr } from "@chakra-ui/react";

import ITable from "@/components/table";
import ITableHeader from "@/components/table/header";
import ITableBody from "@/components/table/body";
  
import { Provider } from "@/canister/declarations/provider_registry/provider_registry.did";
import { hamTableHeader } from "@/constants/contents/ham/table";
import { getProviderStatus } from "@/utils/provider";
import { useProviderStore } from "@/store/providers.store";

interface HAMTableProps {
  datas: Array<Provider>;
  page: number
  limit: number
}

export default function HAMTable({ props }: { props: HAMTableProps}) {
  const { datas, page, limit } = props

  const router = useRouter();
  const params = useSearchParams();
  const pathname = usePathname();
  const { provider } = useProviderStore()

  const onDetail = (data: Provider) => {
    const param = new URLSearchParams(params);
    param.set("hospital", data.V1.internal_id);

    const path = `${pathname}?${param.toString()}`;
    router.push(path);
  };

  return (
    <ITable>
      <ITableHeader headers={hamTableHeader} />
      <ITableBody>
        {datas?.map((data, index) => (
          <Tr
            key={index}
            cursor={"pointer"}
            _hover={{ bg: "primary.100" }}
            onClick={() => onDetail(data)}
            bg={provider?.V1.internal_id === data.V1.internal_id ? "primary.200" : "white"}
          >
            <Td>{(page - 1) * limit + index + 1}</Td>
            <Td>{data.V1.display_name}</Td>
            <Td>{data.V1.address}</Td>
            <Td>{getProviderStatus(data.V1.activation_status)}</Td>
          </Tr>
        ))}
      </ITableBody>
    </ITable>
  );
}
