/* eslint-disable react-hooks/exhaustive-deps */
"use client"

import { patientCanisterId } from "@/config/canisters/patient.canister";
import { PatientActor, usePatientQuery } from "@/services/patients";
import { Flex, Grid, GridItem, Image, Link, Spinner, Text } from "@chakra-ui/react";
import { ReactElement, useEffect, useState } from "react";
import GroupBackNavigation from "./back-navigation";
import { GetUserGroupsResponse, Group } from "@/declarations/patient_registry/patient_registry.did";
import AddGroupModal from "./add";
// import { useUserPrincipal } from "@ic-reactor/react";

const GroupList = (): ReactElement => {
  // const principal = useUserPrincipal()
  const [userGroups, setUserGroups] = useState<Array<Group> | undefined>(undefined)

  const { call: getUserGroups, loading: loadingProviderList } = usePatientQuery({
    functionName: "get_user_groups",
    refetchOnMount: true,
    onSuccess(data) {
      console.log(data)
      const result: GetUserGroupsResponse = data ?? { groups: [] };
      setUserGroups(result.groups);
    },
    onError(error) {
      setUserGroups([])
      console.error(error);
    },
  });

  // useEffect(() => {
  //   console.log("PRINCIPAL Text", principal?.toText());
  // }, [principal]);

  return (
    <Flex
      minHeight={"full"}
      pt={12}
    >
      <Flex
        h={"full"}
        overflowY={"auto"}
        w={"full"}
      >
        <Flex
          w={"full"}
          h={"full"}
          pb={"5.5rem"}
        >
          {
            userGroups 
              ? userGroups.length > 0
                ? <Flex
                  w={"full"}
                  direction={"column"}
                  rowGap={2}
                  pb={4}
                >
                  {
                    userGroups.map((group, index) =>
                      <Link 
                        key={index}
                        href={`/group/${group.id}/member`}
                        textDecoration="none"
                        _hover={{ textDecoration: 'none' }}
                      >
                        <Grid
                          templateColumns="repeat(5, 1fr)"
                          py={2}
                          px={0}
                          columnGap={3}
                          alignItems={"start"}
                        >
                          <GridItem
                            colSpan={1}
                            aspectRatio={1/1}
                            background={"rgb(217, 217, 217)"}
                            display={"block"}
                            rounded={"xl"}
                          />
                          <GridItem 
                            colSpan={4}
                            h={"full"}
                            display={"flex"}
                            flexDirection={"column"}
                            justifyContent={"center"}
                            rowGap={1}
                          >
                            <Text
                              fontWeight={600}
                              fontSize={"lg"}
                              textTransform={"capitalize"}
                            >
                              {group.name} Group
                            </Text>
                            <Text>
                              Total Member {group.members.length}
                            </Text>
                          </GridItem>
                        </Grid>
                      </Link>
                    )
                  }
                </Flex>
                : <NoGroupView />
              : <Flex w={"full"}>
                <Flex
                  w={"full"}
                  margin={"auto"}
                  justifyContent={"center"}
                >
                  <Spinner size={"md"} colorScheme="primary" color="primary.700" />
                </Flex>
              </Flex>
          }
        </Flex>
      </Flex>
      <AddGroupModal  
        props={{
          getUserGroups
        }}
      />
    </Flex>
  )
}

const NoGroupView = () => {
  return (
    <Flex
      w={"full"}
      direction={'column'}
      gap={5}
      justify={'space-between'}
      h={'full'}
      marginY={'auto'}
    >
      <Flex 
        flex={1} 
        direction={'column'} 
        align={'center'} 
        justify={'center'} 
        gap={5}
        h={"full"}
        marginY={'auto'}
      >
        <Image 
          src="/assets/female-doctor.png" 
          alt="No Group" 
          w={"50%"}
        />
        <Flex 
          direction={'column'} 
          align={'center'} 
          gap={1}
        >
          <Text 
            fontSize={'lg'} 
            fontWeight={'bold'} 
            color={'neutral.700'}
          >
            You Don’t Have Group List
          </Text>
          <Text
            fontSize={'sm'}
            color={'neutral.500'}
            textAlign={'center'}
            lineHeight={'1.7'}
          >
            Click “Create Group” To <br />
            Make a Group List
          </Text>
        </Flex>
      </Flex>
    </Flex>
  )
}

export default function PatientGroupList() {
  return (
    <PatientActor canisterId={patientCanisterId}>
      <Flex 
        flex={1}
        w={"full"}
        direction={"column"}
      >
        <GroupBackNavigation />
        <GroupList />
      </Flex>
    </PatientActor>
  )
}