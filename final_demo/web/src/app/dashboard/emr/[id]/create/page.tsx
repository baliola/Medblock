"use client"

import { useRouter } from "next/navigation";
import EMRCreateForm from "@/components/dashboard/emr/create";
import EMRHeader from "@/components/dashboard/emr/header";
import { emrHeader } from "@/constants/contents/dashboard/emr/header";
import { useHospitalStatusStore } from "@/store/hospital-status";

interface PageProps {
  params: {
    id: string;
  }
}

export default function EMRCreatePage({ params }: PageProps) {
  const id = params.id;

  const router = useRouter();
  const hospitalStatus = useHospitalStatusStore(state => state.status);

  if (!id) {
    router.push('/dashboard/patients');
    return;
  }

  if (
    hospitalStatus !== 'idle' &&
    hospitalStatus === 'suspended'
  ) {
    router.push(`/dashboard/emr/${id}`);
    return;
  }

  return (
    <EMRCreateForm
      header={
        <EMRHeader title={emrHeader.create.title} />
      }
    />
  )
}