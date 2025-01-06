import { redirect } from "next/navigation";
import { Divider, Flex } from "@chakra-ui/react";

import EMRPatient from "@/components/emr";
import EMRBackNavigation from "@/components/emr/back-navigation";
import EMRProfile from "@/components/emr/profile";
import { Principal } from "@dfinity/principal";

interface PageProps {
  params: {
    emr_id: string;
    provider_id: string;
    registry_id: any;
  }
}

const isParameterFullfilled = ({
  emr_id,
  provider_id,
  registry_id
}: PageProps['params']) => {
  return !emr_id || !provider_id || !registry_id;
}

const isRegistryIdValid = (registry_id: string) => {
  try {
    Principal.fromText(registry_id);
    return true;
  } catch (error) {
    return false;
  }
}

export default function EMRPage({ params }: PageProps) {
  const { registry_id } = params;

  if (isParameterFullfilled(params)) {
    redirect("/home")
  }

  if (
    registry_id &&
    typeof registry_id === 'string'
  ) {
    if (!isRegistryIdValid(registry_id)) {
      redirect("/home")
    }
  }

  return (
    <Flex w={"full"} direction={'column'} gap={5}>
      <EMRBackNavigation />
      <EMRProfile />
      <Divider />
      <EMRPatient />
    </Flex>
  )
}