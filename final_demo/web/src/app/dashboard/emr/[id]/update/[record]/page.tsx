"use client"

import EMRHeader from "@/components/dashboard/emr/header";
import EMRUpdateForm from "@/components/dashboard/emr/update";
import { emrHeader } from "@/constants/contents/dashboard/emr/header";
import { useHospitalStatusStore } from "@/store/hospital-status";
import { useRouter } from "next/navigation";

interface PageProps {
  params: {
    id: string;
    record: string;
  }
}

const paramFullfilled = (params: PageProps['params']) => {
  if (params.id && params.record) {
    return true;
  }
  return false;
}

export default function EMRUpdatePage({ params }: PageProps) {
  const router = useRouter();
  const hospitalStatus = useHospitalStatusStore(state => state.status);

  if (!paramFullfilled(params)) {
    router.push('/dashboard/patients');
    return;
  }

  if (
    hospitalStatus !== 'idle' &&
    hospitalStatus === 'suspended'
  ) {
    router.push(`/dashboard/emr/${params.id}`);
    return;
  }

  return (
    <EMRUpdateForm
      header={
        <EMRHeader title={emrHeader.update.title} />
      }
    />
  )
}