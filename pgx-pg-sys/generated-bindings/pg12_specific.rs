use crate as pg_sys;
use crate::common::*;
use pgx_macros::*;
#[doc = "\tQuery Tree"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Query {
    pub type_: NodeTag,
    pub commandType: CmdType,
    pub querySource: QuerySource,
    pub queryId: uint64,
    pub canSetTag: bool,
    pub utilityStmt: *mut Node,
    pub resultRelation: ::std::os::raw::c_int,
    pub hasAggs: bool,
    pub hasWindowFuncs: bool,
    pub hasTargetSRFs: bool,
    pub hasSubLinks: bool,
    pub hasDistinctOn: bool,
    pub hasRecursive: bool,
    pub hasModifyingCTE: bool,
    pub hasForUpdate: bool,
    pub hasRowSecurity: bool,
    pub cteList: *mut List,
    pub rtable: *mut List,
    pub jointree: *mut FromExpr,
    pub targetList: *mut List,
    pub override_: OverridingKind,
    pub onConflict: *mut OnConflictExpr,
    pub returningList: *mut List,
    pub groupClause: *mut List,
    pub groupingSets: *mut List,
    pub havingQual: *mut Node,
    pub windowClause: *mut List,
    pub distinctClause: *mut List,
    pub sortClause: *mut List,
    pub limitOffset: *mut Node,
    pub limitCount: *mut Node,
    pub rowMarks: *mut List,
    pub setOperations: *mut Node,
    pub constraintDeps: *mut List,
    pub withCheckOptions: *mut List,
    pub stmt_location: ::std::os::raw::c_int,
    pub stmt_len: ::std::os::raw::c_int,
}
#[pg_guard]
extern "C" {
    pub fn AdvanceNextFullTransactionIdPastXid(xid: TransactionId);
}
#[pg_guard]
extern "C" {
    pub fn AggStateIsShared(fcinfo: FunctionCallInfo) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn AllocSetContextCreateInternal(
        parent: MemoryContext,
        name: *const ::std::os::raw::c_char,
        minContextSize: Size,
        initBlockSize: Size,
        maxBlockSize: Size,
    ) -> MemoryContext;
}
#[pg_guard]
extern "C" {
    pub fn AtEOXact_Enum();
}
#[pg_guard]
extern "C" {
    pub fn AtEOXact_Files(isCommit: bool);
}
#[pg_guard]
extern "C" {
    pub fn BackgroundWorkerInitializeConnection(
        dbname: *const ::std::os::raw::c_char,
        username: *const ::std::os::raw::c_char,
        flags: uint32,
    );
}
#[pg_guard]
extern "C" {
    pub fn BackgroundWorkerInitializeConnectionByOid(dboid: Oid, useroid: Oid, flags: uint32);
}
#[pg_guard]
extern "C" {
    pub fn BasicOpenFile(
        fileName: *const ::std::os::raw::c_char,
        fileFlags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn BasicOpenFilePerm(
        fileName: *const ::std::os::raw::c_char,
        fileFlags: ::std::os::raw::c_int,
        fileMode: mode_t,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn BeginImplicitTransactionBlock();
}
#[pg_guard]
extern "C" {
    pub fn BeginInternalSubTransaction(name: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn BootStrapCLOG();
}
#[pg_guard]
extern "C" {
    pub fn BuildTupleHashTable(
        parent: *mut PlanState,
        inputDesc: TupleDesc,
        numCols: ::std::os::raw::c_int,
        keyColIdx: *mut AttrNumber,
        eqfuncoids: *const Oid,
        hashfunctions: *mut FmgrInfo,
        collations: *mut Oid,
        nbuckets: ::std::os::raw::c_long,
        additionalsize: Size,
        tablecxt: MemoryContext,
        tempcxt: MemoryContext,
        use_variable_hash_iv: bool,
    ) -> TupleHashTable;
}
#[pg_guard]
extern "C" {
    pub fn BuildTupleHashTableExt(
        parent: *mut PlanState,
        inputDesc: TupleDesc,
        numCols: ::std::os::raw::c_int,
        keyColIdx: *mut AttrNumber,
        eqfuncoids: *const Oid,
        hashfunctions: *mut FmgrInfo,
        collations: *mut Oid,
        nbuckets: ::std::os::raw::c_long,
        additionalsize: Size,
        metacxt: MemoryContext,
        tablecxt: MemoryContext,
        tempcxt: MemoryContext,
        use_variable_hash_iv: bool,
    ) -> TupleHashTable;
}
#[pg_guard]
extern "C" {
    pub fn CLOGShmemBuffers() -> Size;
}
#[pg_guard]
extern "C" {
    pub fn CLOGShmemInit();
}
#[pg_guard]
extern "C" {
    pub fn CLOGShmemSize() -> Size;
}
#[pg_guard]
extern "C" {
    pub fn CheckPointCLOG();
}
#[pg_guard]
extern "C" {
    pub fn CompareIndexInfo(
        info1: *mut IndexInfo,
        info2: *mut IndexInfo,
        collations1: *mut Oid,
        collations2: *mut Oid,
        opfamilies1: *mut Oid,
        opfamilies2: *mut Oid,
        attmap: *mut AttrNumber,
        maplen: ::std::os::raw::c_int,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariableBroadcast(cv: *mut ConditionVariable);
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariableInit(cv: *mut ConditionVariable);
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariablePrepareToSleep(cv: *mut ConditionVariable);
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariableSignal(cv: *mut ConditionVariable);
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariableSleep(cv: *mut ConditionVariable, wait_event_info: uint32);
}
#[pg_guard]
extern "C" {
    pub fn CreateAuxProcessResourceOwner();
}
#[pg_guard]
extern "C" {
    pub fn CreateParallelContext(
        library_name: *const ::std::os::raw::c_char,
        function_name: *const ::std::os::raw::c_char,
        nworkers: ::std::os::raw::c_int,
    ) -> *mut ParallelContext;
}
#[pg_guard]
extern "C" {
    pub fn CreateTemplateTupleDesc(natts: ::std::os::raw::c_int) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn CreateTrigger(
        stmt: *mut CreateTrigStmt,
        queryString: *const ::std::os::raw::c_char,
        relOid: Oid,
        refRelOid: Oid,
        constraintOid: Oid,
        indexOid: Oid,
        funcoid: Oid,
        parentTriggerOid: Oid,
        whenClause: *mut Node,
        isInternal: bool,
        in_partition: bool,
    ) -> ObjectAddress;
}
#[pg_guard]
extern "C" {
    pub fn CreateTupleDesc(
        natts: ::std::os::raw::c_int,
        attrs: *mut Form_pg_attribute,
    ) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn DatumGetAnyArrayP(d: Datum) -> *mut AnyArrayType;
}
#[pg_guard]
extern "C" {
    pub fn DefineSavepoint(name: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn EnableDisableTrigger(
        rel: Relation,
        tgname: *const ::std::os::raw::c_char,
        fires_when: ::std::os::raw::c_char,
        skip_system: bool,
        lockmode: LOCKMODE,
    );
}
#[pg_guard]
extern "C" {
    pub fn EndImplicitTransactionBlock();
}
#[pg_guard]
extern "C" {
    pub fn EndTransactionBlock(chain: bool) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn EnumBlacklisted(enum_id: Oid) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn EstimateEnumBlacklistSpace() -> Size;
}
#[pg_guard]
extern "C" {
    pub fn EstimateReindexStateSpace() -> Size;
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQual(
        epqstate: *mut EPQState,
        relation: Relation,
        rti: Index,
        testslot: *mut TupleTableSlot,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualBegin(epqstate: *mut EPQState);
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualFetchRowMark(
        epqstate: *mut EPQState,
        rti: Index,
        slot: *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualInit(
        epqstate: *mut EPQState,
        parentestate: *mut EState,
        subplan: *mut Plan,
        auxrowmarks: *mut List,
        epqParam: ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualSlot(
        epqstate: *mut EPQState,
        relation: Relation,
        rti: Index,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecARInsertTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        slot: *mut TupleTableSlot,
        recheckIndexes: *mut List,
        transition_capture: *mut TransitionCaptureState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecARUpdateTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        tupleid: ItemPointer,
        fdw_trigtuple: HeapTuple,
        slot: *mut TupleTableSlot,
        recheckIndexes: *mut List,
        transition_capture: *mut TransitionCaptureState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecAllocTableSlot(
        tupleTable: *mut *mut List,
        desc: TupleDesc,
        tts_ops: *const TupleTableSlotOps,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecBRDeleteTriggers(
        estate: *mut EState,
        epqstate: *mut EPQState,
        relinfo: *mut ResultRelInfo,
        tupleid: ItemPointer,
        fdw_trigtuple: HeapTuple,
        epqslot: *mut *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecBRInsertTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        slot: *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecBRUpdateTriggers(
        estate: *mut EState,
        epqstate: *mut EPQState,
        relinfo: *mut ResultRelInfo,
        tupleid: ItemPointer,
        fdw_trigtuple: HeapTuple,
        slot: *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecBuildAggTrans(
        aggstate: *mut AggState,
        phase: *mut AggStatePerPhaseData,
        doSort: bool,
        doHash: bool,
    ) -> *mut ExprState;
}
#[pg_guard]
extern "C" {
    pub fn ExecBuildGroupingEqual(
        ldesc: TupleDesc,
        rdesc: TupleDesc,
        lops: *const TupleTableSlotOps,
        rops: *const TupleTableSlotOps,
        numCols: ::std::os::raw::c_int,
        keyColIdx: *const AttrNumber,
        eqfunctions: *const Oid,
        collations: *const Oid,
        parent: *mut PlanState,
    ) -> *mut ExprState;
}
#[pg_guard]
extern "C" {
    pub fn ExecCleanTypeFromTL(targetList: *mut List) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn ExecConditionalAssignProjectionInfo(
        planstate: *mut PlanState,
        inputDesc: TupleDesc,
        varno: Index,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecCreateScanSlotFromOuterPlan(
        estate: *mut EState,
        scanstate: *mut ScanState,
        tts_ops: *const TupleTableSlotOps,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecFetchSlotHeapTuple(
        slot: *mut TupleTableSlot,
        materialize: bool,
        shouldFree: *mut bool,
    ) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn ExecFetchSlotHeapTupleDatum(slot: *mut TupleTableSlot) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn ExecFetchSlotMinimalTuple(
        slot: *mut TupleTableSlot,
        shouldFree: *mut bool,
    ) -> MinimalTuple;
}
#[pg_guard]
extern "C" {
    pub fn ExecForceStoreHeapTuple(tuple: HeapTuple, slot: *mut TupleTableSlot, shouldFree: bool);
}
#[pg_guard]
extern "C" {
    pub fn ExecForceStoreMinimalTuple(
        mtup: MinimalTuple,
        slot: *mut TupleTableSlot,
        shouldFree: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecGetRangeTableRelation(estate: *mut EState, rti: Index) -> Relation;
}
#[pg_guard]
extern "C" {
    pub fn ExecGetResultSlotOps(
        planstate: *mut PlanState,
        isfixed: *mut bool,
    ) -> *const TupleTableSlotOps;
}
#[pg_guard]
extern "C" {
    pub fn ExecGetReturningSlot(
        estate: *mut EState,
        relInfo: *mut ResultRelInfo,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecGetTriggerNewSlot(
        estate: *mut EState,
        relInfo: *mut ResultRelInfo,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecGetTriggerOldSlot(
        estate: *mut EState,
        relInfo: *mut ResultRelInfo,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecIRInsertTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        slot: *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecIRUpdateTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        trigtuple: HeapTuple,
        slot: *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitExprWithParams(node: *mut Expr, ext_params: ParamListInfo) -> *mut ExprState;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitExtraTupleSlot(
        estate: *mut EState,
        tupledesc: TupleDesc,
        tts_ops: *const TupleTableSlotOps,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitJunkFilter(targetList: *mut List, slot: *mut TupleTableSlot) -> *mut JunkFilter;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitNullTupleSlot(
        estate: *mut EState,
        tupType: TupleDesc,
        tts_ops: *const TupleTableSlotOps,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitRangeTable(estate: *mut EState, rangeTable: *mut List);
}
#[pg_guard]
extern "C" {
    pub fn ExecInitResultSlot(planstate: *mut PlanState, tts_ops: *const TupleTableSlotOps);
}
#[pg_guard]
extern "C" {
    pub fn ExecInitResultTupleSlotTL(planstate: *mut PlanState, tts_ops: *const TupleTableSlotOps);
}
#[pg_guard]
extern "C" {
    pub fn ExecInitResultTypeTL(planstate: *mut PlanState);
}
#[pg_guard]
extern "C" {
    pub fn ExecInitScanTupleSlot(
        estate: *mut EState,
        scanstate: *mut ScanState,
        tupleDesc: TupleDesc,
        tts_ops: *const TupleTableSlotOps,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecInsertIndexTuples(
        slot: *mut TupleTableSlot,
        estate: *mut EState,
        noDupErr: bool,
        specConflict: *mut bool,
        arbiterIndexes: *mut List,
    ) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn ExecMakeFunctionResultSet(
        fcache: *mut SetExprState,
        econtext: *mut ExprContext,
        argContext: MemoryContext,
        isNull: *mut bool,
        isDone: *mut ExprDoneCond,
    ) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn ExecPartitionCheck(
        resultRelInfo: *mut ResultRelInfo,
        slot: *mut TupleTableSlot,
        estate: *mut EState,
        emitError: bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecPartitionCheckEmitError(
        resultRelInfo: *mut ResultRelInfo,
        slot: *mut TupleTableSlot,
        estate: *mut EState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecSetExecProcNode(node: *mut PlanState, function: ExecProcNodeMtd);
}
#[pg_guard]
extern "C" {
    pub fn ExecSetTupleBound(tuples_needed: int64, child_node: *mut PlanState);
}
#[pg_guard]
extern "C" {
    pub fn ExecStoreBufferHeapTuple(
        tuple: HeapTuple,
        slot: *mut TupleTableSlot,
        buffer: Buffer,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecStoreHeapTuple(
        tuple: HeapTuple,
        slot: *mut TupleTableSlot,
        shouldFree: bool,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecStoreHeapTupleDatum(data: Datum, slot: *mut TupleTableSlot);
}
#[pg_guard]
extern "C" {
    pub fn ExecStorePinnedBufferHeapTuple(
        tuple: HeapTuple,
        slot: *mut TupleTableSlot,
        buffer: Buffer,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecTypeFromTL(targetList: *mut List) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn ExecuteTruncateGuts(
        explicit_rels: *mut List,
        relids: *mut List,
        relids_logged: *mut List,
        behavior: DropBehavior,
        restart_seqs: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExplainCloseGroup(
        objtype: *const ::std::os::raw::c_char,
        labelname: *const ::std::os::raw::c_char,
        labeled: bool,
        es: *mut ExplainState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExplainOpenGroup(
        objtype: *const ::std::os::raw::c_char,
        labelname: *const ::std::os::raw::c_char,
        labeled: bool,
        es: *mut ExplainState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExplainPrintJIT(
        es: *mut ExplainState,
        jit_flags: ::std::os::raw::c_int,
        jit_instr: *mut JitInstrumentation,
        worker_i: ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExplainPrintJITSummary(es: *mut ExplainState, queryDesc: *mut QueryDesc);
}
#[pg_guard]
extern "C" {
    pub fn ExplainPropertyFloat(
        qlabel: *const ::std::os::raw::c_char,
        unit: *const ::std::os::raw::c_char,
        value: f64,
        ndigits: ::std::os::raw::c_int,
        es: *mut ExplainState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExplainPropertyInteger(
        qlabel: *const ::std::os::raw::c_char,
        unit: *const ::std::os::raw::c_char,
        value: int64,
        es: *mut ExplainState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExtendCLOG(newestXact: TransactionId);
}
#[pg_guard]
extern "C" {
    pub fn FileGetRawMode(file: File) -> mode_t;
}
#[pg_guard]
extern "C" {
    pub fn FileRead(
        file: File,
        buffer: *mut ::std::os::raw::c_char,
        amount: ::std::os::raw::c_int,
        offset: off_t,
        wait_event_info: uint32,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn FileSize(file: File) -> off_t;
}
#[pg_guard]
extern "C" {
    pub fn FileWrite(
        file: File,
        buffer: *mut ::std::os::raw::c_char,
        amount: ::std::os::raw::c_int,
        offset: off_t,
        wait_event_info: uint32,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn FindTupleHashEntry(
        hashtable: TupleHashTable,
        slot: *mut TupleTableSlot,
        eqcomp: *mut ExprState,
        hashfunctions: *mut FmgrInfo,
    ) -> TupleHashEntry;
}
#[pg_guard]
extern "C" {
    pub fn FreeCachedExpression(cexpr: *mut CachedExpression);
}
#[pg_guard]
extern "C" {
    pub fn FunctionCall0Coll(flinfo: *mut FmgrInfo, collation: Oid) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn GenerateTypeDependencies(
        typeObjectId: Oid,
        typeForm: Form_pg_type,
        defaultExpr: *mut Node,
        typacl: *mut ::std::os::raw::c_void,
        relationKind: ::std::os::raw::c_char,
        isImplicitArray: bool,
        isDependentType: bool,
        rebuild: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn GenerationContextCreate(
        parent: MemoryContext,
        name: *const ::std::os::raw::c_char,
        blockSize: Size,
    ) -> MemoryContext;
}
#[pg_guard]
extern "C" {
    pub fn GetBackgroundWorkerTypeByPid(pid: pid_t) -> *const ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn GetCachedExpression(expr: *mut Node) -> *mut CachedExpression;
}
#[pg_guard]
extern "C" {
    pub fn GetCurrentFullTransactionId() -> FullTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn GetCurrentFullTransactionIdIfAny() -> FullTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn GetForeignDataWrapperExtended(fdwid: Oid, flags: bits16) -> *mut ForeignDataWrapper;
}
#[pg_guard]
extern "C" {
    pub fn GetForeignServerExtended(serverid: Oid, flags: bits16) -> *mut ForeignServer;
}
#[pg_guard]
extern "C" {
    pub fn GetFullRecentGlobalXmin() -> FullTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn GetHeapamTableAmRoutine() -> *const TableAmRoutine;
}
#[pg_guard]
extern "C" {
    pub fn GetLockConflicts(
        locktag: *const LOCKTAG,
        lockmode: LOCKMODE,
        countp: *mut ::std::os::raw::c_int,
    ) -> *mut VirtualTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn GetNewTransactionId(isSubXact: bool) -> FullTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn GetSysCacheOid(
        cacheId: ::std::os::raw::c_int,
        oidcol: AttrNumber,
        key1: Datum,
        key2: Datum,
        key3: Datum,
        key4: Datum,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn GetTableAmRoutine(amhandler: Oid) -> *const TableAmRoutine;
}
#[pg_guard]
extern "C" {
    pub fn GetTempTablespaces(
        tableSpaces: *mut Oid,
        numSpaces: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn GetTopFullTransactionId() -> FullTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn GetTopFullTransactionIdIfAny() -> FullTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn HoldPinnedPortals();
}
#[pg_guard]
extern "C" {
    pub fn IndexSetParentIndex(idx: Relation, parentOid: Oid);
}
#[pg_guard]
extern "C" {
    pub fn InitPostgres(
        in_dbname: *const ::std::os::raw::c_char,
        dboid: Oid,
        username: *const ::std::os::raw::c_char,
        useroid: Oid,
        out_dbname: *mut ::std::os::raw::c_char,
        override_allow_connections: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn IsInTransactionBlock(isTopLevel: bool) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn JsonbExtractScalar(jbc: *mut JsonbContainer, res: *mut JsonbValue) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn JsonbHashScalarValueExtended(
        scalarVal: *const JsonbValue,
        hash: *mut uint64,
        seed: uint64,
    );
}
#[pg_guard]
extern "C" {
    pub fn JsonbTypeName(jb: *mut JsonbValue) -> *const ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn LWLockRegisterTranche(
        tranche_id: ::std::os::raw::c_int,
        tranche_name: *const ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn LocalProcessControlFile(reset: bool);
}
#[pg_guard]
extern "C" {
    pub fn LockHeldByMe(locktag: *const LOCKTAG, lockmode: LOCKMODE) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn LookupFuncName(
        funcname: *mut List,
        nargs: ::std::os::raw::c_int,
        argtypes: *const Oid,
        missing_ok: bool,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn LookupFuncWithArgs(
        objtype: ObjectType,
        func: *mut ObjectWithArgs,
        missing_ok: bool,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn MakePGDirectory(directoryName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn MakeSingleTupleTableSlot(
        tupdesc: TupleDesc,
        tts_ops: *const TupleTableSlotOps,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn MakeTupleTableSlot(
        tupleDesc: TupleDesc,
        tts_ops: *const TupleTableSlotOps,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn MemoryContextCreate(
        node: MemoryContext,
        tag: NodeTag,
        methods: *const MemoryContextMethods,
        parent: MemoryContext,
        name: *const ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn MemoryContextSetIdentifier(context: MemoryContext, id: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn OpenTransientFile(
        fileName: *const ::std::os::raw::c_char,
        fileFlags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn OpenTransientFilePerm(
        fileName: *const ::std::os::raw::c_char,
        fileFlags: ::std::os::raw::c_int,
        fileMode: mode_t,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn ParseFuncOrColumn(
        pstate: *mut ParseState,
        funcname: *mut List,
        fargs: *mut List,
        last_srf: *mut Node,
        fn_: *mut FuncCall,
        proc_call: bool,
        location: ::std::os::raw::c_int,
    ) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn PartConstraintImpliedByRelConstraint(
        scanrel: Relation,
        partConstraint: *mut List,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn PathNameCreateTemporaryDir(
        base: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn PathNameCreateTemporaryFile(
        name: *const ::std::os::raw::c_char,
        error_on_failure: bool,
    ) -> File;
}
#[pg_guard]
extern "C" {
    pub fn PathNameDeleteTemporaryDir(name: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn PathNameDeleteTemporaryFile(
        name: *const ::std::os::raw::c_char,
        error_on_failure: bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn PathNameOpenFile(
        fileName: *const ::std::os::raw::c_char,
        fileFlags: ::std::os::raw::c_int,
    ) -> File;
}
#[pg_guard]
extern "C" {
    pub fn PathNameOpenFilePerm(
        fileName: *const ::std::os::raw::c_char,
        fileFlags: ::std::os::raw::c_int,
        fileMode: mode_t,
    ) -> File;
}
#[pg_guard]
extern "C" {
    pub fn PathNameOpenTemporaryFile(name: *const ::std::os::raw::c_char) -> File;
}
#[pg_guard]
extern "C" {
    pub fn PortalErrorCleanup();
}
#[pg_guard]
extern "C" {
    pub fn PrepareTransactionBlock(gid: *const ::std::os::raw::c_char) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn PreventInTransactionBlock(isTopLevel: bool, stmtType: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn RI_FKey_fk_upd_check_required(
        trigger: *mut Trigger,
        fk_rel: Relation,
        old_slot: *mut TupleTableSlot,
        new_slot: *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn RI_FKey_pk_upd_check_required(
        trigger: *mut Trigger,
        pk_rel: Relation,
        old_slot: *mut TupleTableSlot,
        new_slot: *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn RI_PartitionRemove_Check(trigger: *mut Trigger, fk_rel: Relation, pk_rel: Relation);
}
#[pg_guard]
extern "C" {
    pub fn RangeVarGetRelidExtended(
        relation: *const RangeVar,
        lockmode: LOCKMODE,
        flags: uint32,
        callback: RangeVarGetRelidCallback,
        callback_arg: *mut ::std::os::raw::c_void,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn ReadNextFullTransactionId() -> FullTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn RelationBuildLocalRelation(
        relname: *const ::std::os::raw::c_char,
        relnamespace: Oid,
        tupDesc: TupleDesc,
        relid: Oid,
        accessmtd: Oid,
        relfilenode: Oid,
        reltablespace: Oid,
        shared_relation: bool,
        mapped_relation: bool,
        relpersistence: ::std::os::raw::c_char,
        relkind: ::std::os::raw::c_char,
    ) -> Relation;
}
#[pg_guard]
extern "C" {
    pub fn RelationInitTableAccessMethod(relation: Relation);
}
#[pg_guard]
extern "C" {
    pub fn RelationSetNewRelfilenode(relation: Relation, persistence: ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn ReleaseAuxProcessResources(isCommit: bool);
}
#[pg_guard]
extern "C" {
    pub fn ReleaseSavepoint(name: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn RemoveEventTriggerById(trigOid: Oid);
}
#[pg_guard]
extern "C" {
    pub fn RenameRelationInternal(
        myrelid: Oid,
        newrelname: *const ::std::os::raw::c_char,
        is_internal: bool,
        is_index: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn RenameTypeInternal(
        typeOid: Oid,
        newTypeName: *const ::std::os::raw::c_char,
        typeNamespace: Oid,
    );
}
#[pg_guard]
extern "C" {
    pub fn RequireTransactionBlock(isTopLevel: bool, stmtType: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn ResetTupleHashTable(hashtable: TupleHashTable);
}
#[pg_guard]
extern "C" {
    pub fn RestoreEnumBlacklist(space: *mut ::std::os::raw::c_void);
}
#[pg_guard]
extern "C" {
    pub fn RestoreReindexState(reindexstate: *mut ::std::os::raw::c_void);
}
#[pg_guard]
extern "C" {
    pub fn RestoreTransactionCharacteristics();
}
#[pg_guard]
extern "C" {
    pub fn RollbackToSavepoint(name: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn SPICleanup();
}
#[pg_guard]
extern "C" {
    pub fn SPI_commit();
}
#[pg_guard]
extern "C" {
    pub fn SPI_commit_and_chain();
}
#[pg_guard]
extern "C" {
    pub fn SPI_connect_ext(options: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn SPI_inside_nonatomic_context() -> bool;
}
#[pg_guard]
extern "C" {
    pub fn SPI_rollback();
}
#[pg_guard]
extern "C" {
    pub fn SPI_rollback_and_chain();
}
#[pg_guard]
extern "C" {
    pub fn SPI_start_transaction();
}
#[pg_guard]
extern "C" {
    pub fn SaveTransactionCharacteristics();
}
#[pg_guard]
extern "C" {
    pub fn SearchSysCache1(cacheId: ::std::os::raw::c_int, key1: Datum) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn SearchSysCache2(cacheId: ::std::os::raw::c_int, key1: Datum, key2: Datum) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn SearchSysCache3(
        cacheId: ::std::os::raw::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
    ) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn SearchSysCache4(
        cacheId: ::std::os::raw::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
        key4: Datum,
    ) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn SearchSysCacheAttNum(relid: Oid, attnum: int16) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn SearchSysCacheCopyAttNum(relid: Oid, attnum: int16) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn SearchSysCacheList(
        cacheId: ::std::os::raw::c_int,
        nkeys: ::std::os::raw::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
    ) -> *mut catclist;
}
#[pg_guard]
extern "C" {
    pub fn SerializeEnumBlacklist(space: *mut ::std::os::raw::c_void, size: Size);
}
#[pg_guard]
extern "C" {
    pub fn SerializeReindexState(maxsize: Size, start_address: *mut ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn SharedFileSetAttach(fileset: *mut SharedFileSet, seg: *mut dsm_segment);
}
#[pg_guard]
extern "C" {
    pub fn SharedFileSetCreate(
        fileset: *mut SharedFileSet,
        name: *const ::std::os::raw::c_char,
    ) -> File;
}
#[pg_guard]
extern "C" {
    pub fn SharedFileSetDelete(
        fileset: *mut SharedFileSet,
        name: *const ::std::os::raw::c_char,
        error_on_failure: bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn SharedFileSetDeleteAll(fileset: *mut SharedFileSet);
}
#[pg_guard]
extern "C" {
    pub fn SharedFileSetInit(fileset: *mut SharedFileSet, seg: *mut dsm_segment);
}
#[pg_guard]
extern "C" {
    pub fn SharedFileSetOpen(
        fileset: *mut SharedFileSet,
        name: *const ::std::os::raw::c_char,
    ) -> File;
}
#[pg_guard]
extern "C" {
    pub fn SharedRecordTypmodRegistryAttach(arg1: *mut SharedRecordTypmodRegistry);
}
#[pg_guard]
extern "C" {
    pub fn SharedRecordTypmodRegistryEstimate() -> usize;
}
#[pg_guard]
extern "C" {
    pub fn SharedRecordTypmodRegistryInit(
        arg1: *mut SharedRecordTypmodRegistry,
        segment: *mut dsm_segment,
        area: *mut dsa_area,
    );
}
#[pg_guard]
extern "C" {
    pub fn ShutdownCLOG();
}
#[pg_guard]
extern "C" {
    pub fn StandbyReleaseOldLocks(oldxid: TransactionId);
}
#[pg_guard]
extern "C" {
    pub fn StartupCLOG();
}
#[pg_guard]
extern "C" {
    pub fn TempTablespacePath(path: *mut ::std::os::raw::c_char, tablespace: Oid);
}
#[pg_guard]
extern "C" {
    pub fn TemporalSimplify(max_precis: int32, node: *mut Node) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn TimestampTimestampTzRequiresRewrite() -> bool;
}
#[pg_guard]
extern "C" {
    pub fn TransactionIdGetStatus(xid: TransactionId, lsn: *mut XLogRecPtr) -> XidStatus;
}
#[pg_guard]
extern "C" {
    pub fn TransactionIdSetTreeStatus(
        xid: TransactionId,
        nsubxids: ::std::os::raw::c_int,
        subxids: *mut TransactionId,
        status: XidStatus,
        lsn: XLogRecPtr,
    );
}
#[pg_guard]
extern "C" {
    pub fn TrimCLOG();
}
#[pg_guard]
extern "C" {
    pub fn TruncateCLOG(oldestXact: TransactionId, oldestxid_datoid: Oid);
}
#[pg_guard]
extern "C" {
    pub fn TupleDescCopy(dst: TupleDesc, src: TupleDesc);
}
#[pg_guard]
extern "C" {
    pub fn TypeCreate(
        newTypeOid: Oid,
        typeName: *const ::std::os::raw::c_char,
        typeNamespace: Oid,
        relationOid: Oid,
        relationKind: ::std::os::raw::c_char,
        ownerId: Oid,
        internalSize: int16,
        typeType: ::std::os::raw::c_char,
        typeCategory: ::std::os::raw::c_char,
        typePreferred: bool,
        typDelim: ::std::os::raw::c_char,
        inputProcedure: Oid,
        outputProcedure: Oid,
        receiveProcedure: Oid,
        sendProcedure: Oid,
        typmodinProcedure: Oid,
        typmodoutProcedure: Oid,
        analyzeProcedure: Oid,
        elementType: Oid,
        isImplicitArray: bool,
        arrayType: Oid,
        baseType: Oid,
        defaultTypeValue: *const ::std::os::raw::c_char,
        defaultTypeBin: *mut ::std::os::raw::c_char,
        passedByValue: bool,
        alignment: ::std::os::raw::c_char,
        storage: ::std::os::raw::c_char,
        typeMod: int32,
        typNDims: int32,
        typeNotNull: bool,
        typeCollation: Oid,
    ) -> ObjectAddress;
}
#[pg_guard]
extern "C" {
    pub fn TypeShellMake(
        typeName: *const ::std::os::raw::c_char,
        typeNamespace: Oid,
        ownerId: Oid,
    ) -> ObjectAddress;
}
#[pg_guard]
extern "C" {
    pub fn UserAbortTransactionBlock(chain: bool);
}
#[pg_guard]
extern "C" {
    pub fn WaitForParallelWorkersToAttach(pcxt: *mut ParallelContext);
}
#[pg_guard]
extern "C" {
    pub fn WarnNoTransactionBlock(isTopLevel: bool, stmtType: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn XLogReaderAllocate(
        wal_segment_size: ::std::os::raw::c_int,
        pagereadfunc: XLogPageReadCB,
        private_data: *mut ::std::os::raw::c_void,
    ) -> *mut XLogReaderState;
}
#[pg_guard]
extern "C" {
    pub fn XactLogAbortRecord(
        abort_time: TimestampTz,
        nsubxacts: ::std::os::raw::c_int,
        subxacts: *mut TransactionId,
        nrels: ::std::os::raw::c_int,
        rels: *mut RelFileNode,
        xactflags: ::std::os::raw::c_int,
        twophase_xid: TransactionId,
        twophase_gid: *const ::std::os::raw::c_char,
    ) -> XLogRecPtr;
}
#[pg_guard]
extern "C" {
    pub fn XactLogCommitRecord(
        commit_time: TimestampTz,
        nsubxacts: ::std::os::raw::c_int,
        subxacts: *mut TransactionId,
        nrels: ::std::os::raw::c_int,
        rels: *mut RelFileNode,
        nmsgs: ::std::os::raw::c_int,
        msgs: *mut SharedInvalidationMessage,
        relcacheInval: bool,
        forceSync: bool,
        xactflags: ::std::os::raw::c_int,
        twophase_xid: TransactionId,
        twophase_gid: *const ::std::os::raw::c_char,
    ) -> XLogRecPtr;
}
#[pg_guard]
extern "C" {
    pub fn XidInMVCCSnapshot(xid: TransactionId, snapshot: Snapshot) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn aclcheck_error(
        aclerr: AclResult,
        objtype: ObjectType,
        objectname: *const ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn aclcheck_error_col(
        aclerr: AclResult,
        objtype: ObjectType,
        objectname: *const ::std::os::raw::c_char,
        colname: *const ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn acldefault(objtype: ObjectType, ownerId: Oid) -> *mut Acl;
}
#[pg_guard]
extern "C" {
    pub fn add_bool_reloption(
        kinds: bits32,
        name: *const ::std::os::raw::c_char,
        desc: *const ::std::os::raw::c_char,
        default_val: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn add_child_join_rel_equivalences(
        root: *mut PlannerInfo,
        nappinfos: ::std::os::raw::c_int,
        appinfos: *mut *mut AppendRelInfo,
        parent_rel: *mut RelOptInfo,
        child_rel: *mut RelOptInfo,
    );
}
#[pg_guard]
extern "C" {
    pub fn add_int_reloption(
        kinds: bits32,
        name: *const ::std::os::raw::c_char,
        desc: *const ::std::os::raw::c_char,
        default_val: ::std::os::raw::c_int,
        min_val: ::std::os::raw::c_int,
        max_val: ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn add_paths_to_append_rel(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        live_childrels: *mut List,
    );
}
#[pg_guard]
extern "C" {
    pub fn add_predicate_to_index_quals(
        index: *mut IndexOptInfo,
        indexQuals: *mut List,
    ) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn add_real_reloption(
        kinds: bits32,
        name: *const ::std::os::raw::c_char,
        desc: *const ::std::os::raw::c_char,
        default_val: f64,
        min_val: f64,
        max_val: f64,
    );
}
#[pg_guard]
extern "C" {
    pub fn add_string_reloption(
        kinds: bits32,
        name: *const ::std::os::raw::c_char,
        desc: *const ::std::os::raw::c_char,
        default_val: *const ::std::os::raw::c_char,
        validator: validate_string_relopt,
    );
}
#[pg_guard]
extern "C" {
    pub fn adjust_limit_rows_costs(
        rows: *mut f64,
        startup_cost: *mut Cost,
        total_cost: *mut Cost,
        offset_est: int64,
        count_est: int64,
    );
}
#[pg_guard]
extern "C" {
    pub fn appendBinaryStringInfoNT(
        str: StringInfo,
        data: *const ::std::os::raw::c_char,
        datalen: ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn array_map(
        arrayd: Datum,
        exprstate: *mut ExprState,
        econtext: *mut ExprContext,
        retType: Oid,
        amstate: *mut ArrayMapState,
    ) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn array_unnest_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn assign_record_type_identifier(type_id: Oid, typmod: int32) -> uint64;
}
#[pg_guard]
extern "C" {
    pub fn begin_tup_output_tupdesc(
        dest: *mut DestReceiver,
        tupdesc: TupleDesc,
        tts_ops: *const TupleTableSlotOps,
    ) -> *mut TupOutputState;
}
#[pg_guard]
extern "C" {
    pub fn binary_upgrade_set_missing_value(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn bms_add_range(
        a: *mut Bitmapset,
        lower: ::std::os::raw::c_int,
        upper: ::std::os::raw::c_int,
    ) -> *mut Bitmapset;
}
#[pg_guard]
extern "C" {
    pub fn bms_compare(a: *const Bitmapset, b: *const Bitmapset) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn bms_member_index(a: *mut Bitmapset, x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn bms_prev_member(
        a: *const Bitmapset,
        prevbit: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn btnametextcmp(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn bttextnamecmp(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn build_child_join_rel(
        root: *mut PlannerInfo,
        outer_rel: *mut RelOptInfo,
        inner_rel: *mut RelOptInfo,
        parent_joinrel: *mut RelOptInfo,
        restrictlist: *mut List,
        sjinfo: *mut SpecialJoinInfo,
        jointype: JoinType,
    ) -> *mut RelOptInfo;
}
#[pg_guard]
extern "C" {
    pub fn build_function_result_tupdesc_d(
        prokind: ::std::os::raw::c_char,
        proallargtypes: Datum,
        proargmodes: Datum,
        proargnames: Datum,
    ) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn build_partition_pathkeys(
        root: *mut PlannerInfo,
        partrel: *mut RelOptInfo,
        scandir: ScanDirection,
        partialkeys: *mut bool,
    ) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn calc_nestloop_required_outer(
        outerrelids: Relids,
        outer_paramrels: Relids,
        innerrelids: Relids,
        inner_paramrels: Relids,
    ) -> Relids;
}
#[pg_guard]
extern "C" {
    pub fn canonicalize_qual(qual: *mut Expr, is_check: bool) -> *mut Expr;
}
#[pg_guard]
extern "C" {
    pub fn changeDependenciesOf(
        classId: Oid,
        oldObjectId: Oid,
        newObjectId: Oid,
    ) -> ::std::os::raw::c_long;
}
#[pg_guard]
extern "C" {
    pub fn changeDependenciesOn(
        refClassId: Oid,
        oldRefObjectId: Oid,
        newRefObjectId: Oid,
    ) -> ::std::os::raw::c_long;
}
#[pg_guard]
extern "C" {
    pub fn checkDataDir();
}
#[pg_guard]
extern "C" {
    pub fn checkTempNamespaceStatus(namespaceId: Oid) -> TempNamespaceStatus;
}
#[pg_guard]
extern "C" {
    pub fn check_default_table_access_method(
        newval: *mut *mut ::std::os::raw::c_char,
        extra: *mut *mut ::std::os::raw::c_void,
        source: GucSource,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn clauselist_selectivity_simple(
        root: *mut PlannerInfo,
        clauses: *mut List,
        varRelid: ::std::os::raw::c_int,
        jointype: JoinType,
        sjinfo: *mut SpecialJoinInfo,
        estimatedclauses: *mut Bitmapset,
    ) -> Selectivity;
}
#[pg_guard]
extern "C" {
    pub fn clog_desc(buf: StringInfo, record: *mut XLogReaderState);
}
#[pg_guard]
extern "C" {
    pub fn clog_identify(info: uint8) -> *const ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn clog_redo(record: *mut XLogReaderState);
}
#[pg_guard]
extern "C" {
    pub fn commute_restrictinfo(rinfo: *mut RestrictInfo, comm_op: Oid) -> *mut RestrictInfo;
}
#[pg_guard]
extern "C" {
    pub fn compute_parallel_worker(
        rel: *mut RelOptInfo,
        heap_pages: f64,
        index_pages: f64,
        max_workers: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn compute_semi_anti_join_factors(
        root: *mut PlannerInfo,
        joinrel: *mut RelOptInfo,
        outerrel: *mut RelOptInfo,
        innerrel: *mut RelOptInfo,
        jointype: JoinType,
        sjinfo: *mut SpecialJoinInfo,
        restrictlist: *mut List,
        semifactors: *mut SemiAntiJoinFactors,
    );
}
#[pg_guard]
extern "C" {
    pub fn contain_var_clause(node: *mut Node) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn contain_vars_of_level(node: *mut Node, levelsup: ::std::os::raw::c_int) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn convert_tuples_by_name_map_if_req(
        indesc: TupleDesc,
        outdesc: TupleDesc,
        msg: *const ::std::os::raw::c_char,
    ) -> *mut AttrNumber;
}
#[pg_guard]
extern "C" {
    pub fn cost_agg(
        path: *mut Path,
        root: *mut PlannerInfo,
        aggstrategy: AggStrategy,
        aggcosts: *const AggClauseCosts,
        numGroupCols: ::std::os::raw::c_int,
        numGroups: f64,
        quals: *mut List,
        input_startup_cost: Cost,
        input_total_cost: Cost,
        input_tuples: f64,
    );
}
#[pg_guard]
extern "C" {
    pub fn cost_append(path: *mut AppendPath);
}
#[pg_guard]
extern "C" {
    pub fn cost_group(
        path: *mut Path,
        root: *mut PlannerInfo,
        numGroupCols: ::std::os::raw::c_int,
        numGroups: f64,
        quals: *mut List,
        input_startup_cost: Cost,
        input_total_cost: Cost,
        input_tuples: f64,
    );
}
#[pg_guard]
extern "C" {
    pub fn cost_resultscan(
        path: *mut Path,
        root: *mut PlannerInfo,
        baserel: *mut RelOptInfo,
        param_info: *mut ParamPathInfo,
    );
}
#[pg_guard]
extern "C" {
    pub fn count_nonjunk_tlist_entries(tlist: *mut List) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn create_append_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        subpaths: *mut List,
        partial_subpaths: *mut List,
        pathkeys: *mut List,
        required_outer: Relids,
        parallel_workers: ::std::os::raw::c_int,
        parallel_aware: bool,
        partitioned_rels: *mut List,
        rows: f64,
    ) -> *mut AppendPath;
}
#[pg_guard]
extern "C" {
    pub fn create_foreign_join_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        target: *mut PathTarget,
        rows: f64,
        startup_cost: Cost,
        total_cost: Cost,
        pathkeys: *mut List,
        required_outer: Relids,
        fdw_outerpath: *mut Path,
        fdw_private: *mut List,
    ) -> *mut ForeignPath;
}
#[pg_guard]
extern "C" {
    pub fn create_foreign_upper_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        target: *mut PathTarget,
        rows: f64,
        startup_cost: Cost,
        total_cost: Cost,
        pathkeys: *mut List,
        fdw_outerpath: *mut Path,
        fdw_private: *mut List,
    ) -> *mut ForeignPath;
}
#[pg_guard]
extern "C" {
    pub fn create_group_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        subpath: *mut Path,
        groupClause: *mut List,
        qual: *mut List,
        numGroups: f64,
    ) -> *mut GroupPath;
}
#[pg_guard]
extern "C" {
    pub fn create_group_result_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        target: *mut PathTarget,
        havingqual: *mut List,
    ) -> *mut GroupResultPath;
}
#[pg_guard]
extern "C" {
    pub fn create_groupingsets_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        subpath: *mut Path,
        having_qual: *mut List,
        aggstrategy: AggStrategy,
        rollups: *mut List,
        agg_costs: *const AggClauseCosts,
        numGroups: f64,
    ) -> *mut GroupingSetsPath;
}
#[pg_guard]
extern "C" {
    pub fn create_hashjoin_path(
        root: *mut PlannerInfo,
        joinrel: *mut RelOptInfo,
        jointype: JoinType,
        workspace: *mut JoinCostWorkspace,
        extra: *mut JoinPathExtraData,
        outer_path: *mut Path,
        inner_path: *mut Path,
        parallel_hash: bool,
        restrict_clauses: *mut List,
        required_outer: Relids,
        hashclauses: *mut List,
    ) -> *mut HashPath;
}
#[pg_guard]
extern "C" {
    pub fn create_index_path(
        root: *mut PlannerInfo,
        index: *mut IndexOptInfo,
        indexclauses: *mut List,
        indexorderbys: *mut List,
        indexorderbycols: *mut List,
        pathkeys: *mut List,
        indexscandir: ScanDirection,
        indexonly: bool,
        required_outer: Relids,
        loop_count: f64,
        partial_path: bool,
    ) -> *mut IndexPath;
}
#[pg_guard]
extern "C" {
    pub fn create_modifytable_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        operation: CmdType,
        canSetTag: bool,
        nominalRelation: Index,
        rootRelation: Index,
        partColsUpdated: bool,
        resultRelations: *mut List,
        subpaths: *mut List,
        subroots: *mut List,
        withCheckOptionLists: *mut List,
        returningLists: *mut List,
        rowMarks: *mut List,
        onconflict: *mut OnConflictExpr,
        epqParam: ::std::os::raw::c_int,
    ) -> *mut ModifyTablePath;
}
#[pg_guard]
extern "C" {
    pub fn create_resultscan_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        required_outer: Relids,
    ) -> *mut Path;
}
#[pg_guard]
extern "C" {
    pub fn create_windowagg_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        subpath: *mut Path,
        target: *mut PathTarget,
        windowFuncs: *mut List,
        winclause: *mut WindowClause,
    ) -> *mut WindowAggPath;
}
#[pg_guard]
extern "C" {
    pub fn dacosh(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn dasinh(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn datanh(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn dcosh(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn do_pg_start_backup(
        backupidstr: *const ::std::os::raw::c_char,
        fast: bool,
        starttli_p: *mut TimeLineID,
        labelfile: StringInfo,
        tablespaces: *mut *mut List,
        tblspcmapfile: StringInfo,
        infotbssize: bool,
        needtblspcmapfile: bool,
    ) -> XLogRecPtr;
}
#[pg_guard]
extern "C" {
    pub fn dsinh(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn dtanh(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn estimate_hash_bucket_stats(
        root: *mut PlannerInfo,
        hashkey: *mut Node,
        nbuckets: f64,
        mcv_freq: *mut Selectivity,
        bucketsize_frac: *mut Selectivity,
    );
}
#[pg_guard]
extern "C" {
    pub fn estimate_hashagg_tablesize(
        path: *mut Path,
        agg_costs: *const AggClauseCosts,
        dNumGroups: f64,
    ) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn evaluate_expr(
        expr: *mut Expr,
        result_type: Oid,
        result_typmod: int32,
        result_collation: Oid,
    ) -> *mut Expr;
}
#[pg_guard]
extern "C" {
    pub fn execTuplesHashPrepare(
        numCols: ::std::os::raw::c_int,
        eqOperators: *const Oid,
        eqFuncOids: *mut *mut Oid,
        hashFunctions: *mut *mut FmgrInfo,
    );
}
#[pg_guard]
extern "C" {
    pub fn execTuplesMatchPrepare(
        desc: TupleDesc,
        numCols: ::std::os::raw::c_int,
        keyColIdx: *const AttrNumber,
        eqOperators: *const Oid,
        collations: *const Oid,
        parent: *mut PlanState,
    ) -> *mut ExprState;
}
#[pg_guard]
extern "C" {
    pub fn execute_attr_map_slot(
        attrMap: *mut AttrNumber,
        in_slot: *mut TupleTableSlot,
        out_slot: *mut TupleTableSlot,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn execute_attr_map_tuple(tuple: HeapTuple, map: *mut TupleConversionMap) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn expand_function_arguments(
        args: *mut List,
        result_type: Oid,
        func_tuple: *mut HeapTupleData,
    ) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn expand_planner_arrays(root: *mut PlannerInfo, add_size: ::std::os::raw::c_int);
}
#[pg_guard]
extern "C" {
    pub fn expression_planner_with_deps(
        expr: *mut Expr,
        relationOids: *mut *mut List,
        invalItems: *mut *mut List,
    ) -> *mut Expr;
}
#[pg_guard]
extern "C" {
    pub fn expression_returns_set_rows(root: *mut PlannerInfo, clause: *mut Node) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn extract_query_dependencies(
        query: *mut Node,
        relationOids: *mut *mut List,
        invalItems: *mut *mut List,
        hasRowSecurity: *mut bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn find_param_path_info(rel: *mut RelOptInfo, required_outer: Relids)
        -> *mut ParamPathInfo;
}
#[pg_guard]
extern "C" {
    pub fn flatten_join_alias_vars(query: *mut Query, node: *mut Node) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn fmgr_symbol(
        functionId: Oid,
        mod_: *mut *mut ::std::os::raw::c_char,
        fn_: *mut *mut ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn format_type_extended(
        type_oid: Oid,
        typemod: int32,
        flags: bits16,
    ) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn generate_gather_paths(root: *mut PlannerInfo, rel: *mut RelOptInfo, override_rows: bool);
}
#[pg_guard]
extern "C" {
    pub fn generate_partitionwise_join_paths(root: *mut PlannerInfo, rel: *mut RelOptInfo);
}
#[pg_guard]
extern "C" {
    pub fn generate_series_int4_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn generate_series_int8_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn genericcostestimate(
        root: *mut PlannerInfo,
        path: *mut IndexPath,
        loop_count: f64,
        costs: *mut GenericCosts,
    );
}
#[pg_guard]
extern "C" {
    pub fn get_attgenerated(relid: Oid, attnum: AttrNumber) -> ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn get_attname(
        relid: Oid,
        attnum: AttrNumber,
        missing_ok: bool,
    ) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn get_catalog_object_by_oid(
        catalog: Relation,
        oidcol: AttrNumber,
        objectId: Oid,
    ) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn get_collation_isdeterministic(colloid: Oid) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn get_expr_result_tupdesc(expr: *mut Node, noError: bool) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn get_func_prokind(funcid: Oid) -> ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn get_func_support(funcid: Oid) -> RegProcedure;
}
#[pg_guard]
extern "C" {
    pub fn get_index_column_opclass(index_oid: Oid, attno: ::std::os::raw::c_int) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn get_index_isvalid(index_oid: Oid) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn get_index_ref_constraints(indexId: Oid) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn get_object_attnum_oid(class_id: Oid) -> AttrNumber;
}
#[pg_guard]
extern "C" {
    pub fn get_object_type(class_id: Oid, object_id: Oid) -> ObjectType;
}
#[pg_guard]
extern "C" {
    pub fn get_opclass_opfamily_and_input_type(
        opclass: Oid,
        opfamily: *mut Oid,
        opcintype: *mut Oid,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn get_publication_name(pubid: Oid, missing_ok: bool) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn get_quals_from_indexclauses(indexclauses: *mut List) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn get_relkind_objtype(relkind: ::std::os::raw::c_char) -> ObjectType;
}
#[pg_guard]
extern "C" {
    pub fn get_sortgroupclause_expr(
        sgClause: *mut SortGroupClause,
        targetList: *mut List,
    ) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn get_sortgroupclause_tle(
        sgClause: *mut SortGroupClause,
        targetList: *mut List,
    ) -> *mut TargetEntry;
}
#[pg_guard]
extern "C" {
    pub fn get_sortgrouplist_exprs(sgClauses: *mut List, targetList: *mut List) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn get_sortgroupref_clause(sortref: Index, clauses: *mut List) -> *mut SortGroupClause;
}
#[pg_guard]
extern "C" {
    pub fn get_sortgroupref_clause_noerr(
        sortref: Index,
        clauses: *mut List,
    ) -> *mut SortGroupClause;
}
#[pg_guard]
extern "C" {
    pub fn get_sortgroupref_tle(sortref: Index, targetList: *mut List) -> *mut TargetEntry;
}
#[pg_guard]
extern "C" {
    pub fn get_user_default_acl(objtype: ObjectType, ownerId: Oid, nsp_oid: Oid) -> *mut Acl;
}
#[pg_guard]
extern "C" {
    pub fn getmissingattr(
        tupleDesc: TupleDesc,
        attnum: ::std::os::raw::c_int,
        isnull: *mut bool,
    ) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashTupleDesc(tupdesc: TupleDesc) -> uint32;
}
#[pg_guard]
extern "C" {
    pub fn hash_aclitem_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hash_array_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hash_numeric_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hash_range_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashbpcharextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashcharextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashenumextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashfloat4extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashfloat8extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashinetextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashint2extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashint4extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashint8extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashmacaddr8extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashmacaddrextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashnameextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashoidextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashoidvectorextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashtextextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashtid(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashtidextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn hashvarlenaextended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn have_partkey_equi_join(
        joinrel: *mut RelOptInfo,
        rel1: *mut RelOptInfo,
        rel2: *mut RelOptInfo,
        jointype: JoinType,
        restrictlist: *mut List,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_attisnull(
        tup: HeapTuple,
        attnum: ::std::os::raw::c_int,
        tupleDesc: TupleDesc,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_expand_tuple(sourceTuple: HeapTuple, tupleDesc: TupleDesc) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn heap_tableam_handler(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_date_interval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_float4_float8(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_float8_float8(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_int2_int2(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_int2_int4(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_int2_int8(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_int4_int2(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_int4_int4(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_int4_int8(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_int8_int8(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_interval_interval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_numeric_numeric(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_time_interval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_timestamp_interval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_timestamptz_interval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn in_range_timetz_interval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn index_build(
        heapRelation: Relation,
        indexRelation: Relation,
        indexInfo: *mut IndexInfo,
        isreindex: bool,
        parallel: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn index_compute_xid_horizon_for_tuples(
        irel: Relation,
        hrel: Relation,
        ibuf: Buffer,
        itemnos: *mut OffsetNumber,
        nitems: ::std::os::raw::c_int,
    ) -> TransactionId;
}
#[pg_guard]
extern "C" {
    pub fn index_concurrently_build(heapRelationId: Oid, indexRelationId: Oid);
}
#[pg_guard]
extern "C" {
    pub fn index_concurrently_create_copy(
        heapRelation: Relation,
        oldIndexId: Oid,
        newName: *const ::std::os::raw::c_char,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn index_concurrently_set_dead(heapId: Oid, indexId: Oid);
}
#[pg_guard]
extern "C" {
    pub fn index_concurrently_swap(
        newIndexId: Oid,
        oldIndexId: Oid,
        oldName: *const ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn index_constraint_create(
        heapRelation: Relation,
        indexRelationId: Oid,
        parentConstraintId: Oid,
        indexInfo: *mut IndexInfo,
        constraintName: *const ::std::os::raw::c_char,
        constraintType: ::std::os::raw::c_char,
        constr_flags: bits16,
        allow_system_table_mods: bool,
        is_internal: bool,
    ) -> ObjectAddress;
}
#[pg_guard]
extern "C" {
    pub fn index_create(
        heapRelation: Relation,
        indexRelationName: *const ::std::os::raw::c_char,
        indexRelationId: Oid,
        parentIndexRelid: Oid,
        parentConstraintId: Oid,
        relFileNode: Oid,
        indexInfo: *mut IndexInfo,
        indexColNames: *mut List,
        accessMethodObjectId: Oid,
        tableSpaceId: Oid,
        collationObjectId: *mut Oid,
        classObjectId: *mut Oid,
        coloptions: *mut int16,
        reloptions: Datum,
        flags: bits16,
        constr_flags: bits16,
        allow_system_table_mods: bool,
        is_internal: bool,
        constraintId: *mut Oid,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn index_drop(indexId: Oid, concurrent: bool, concurrent_lock_mode: bool);
}
#[pg_guard]
extern "C" {
    pub fn index_fetch_heap(scan: IndexScanDesc, slot: *mut TupleTableSlot) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn index_getnext_slot(
        scan: IndexScanDesc,
        direction: ScanDirection,
        slot: *mut TupleTableSlot,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn index_other_operands_eval_cost(root: *mut PlannerInfo, indexquals: *mut List) -> Cost;
}
#[pg_guard]
extern "C" {
    pub fn index_store_float8_orderby_distances(
        scan: IndexScanDesc,
        orderByTypes: *mut Oid,
        distances: *mut IndexOrderByDistance,
        recheckOrderBy: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn index_truncate_tuple(
        sourceDescriptor: TupleDesc,
        source: IndexTuple,
        leavenatts: ::std::os::raw::c_int,
    ) -> IndexTuple;
}
#[pg_guard]
extern "C" {
    pub fn ineq_histogram_selectivity(
        root: *mut PlannerInfo,
        vardata: *mut VariableStatData,
        opproc: *mut FmgrInfo,
        isgt: bool,
        iseq: bool,
        constval: Datum,
        consttype: Oid,
    ) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn initial_cost_hashjoin(
        root: *mut PlannerInfo,
        workspace: *mut JoinCostWorkspace,
        jointype: JoinType,
        hashclauses: *mut List,
        outer_path: *mut Path,
        inner_path: *mut Path,
        extra: *mut JoinPathExtraData,
        parallel_hash: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn interval_hash_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn interval_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn isTempNamespaceInUse(namespaceId: Oid) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn is_pseudo_constant_for_index(expr: *mut Node, index: *mut IndexOptInfo) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn is_redundant_with_indexclauses(
        rinfo: *mut RestrictInfo,
        indexclauses: *mut List,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn json_string_to_tsvector(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn json_string_to_tsvector_byid(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_bool(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_float4(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_float8(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_hash_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_int2(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_int4(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_int8(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_numeric(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_path_exists(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_path_exists_opr(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_path_match(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_path_match_opr(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_path_query(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_path_query_array(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_path_query_first(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_string_to_tsvector(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonb_string_to_tsvector_byid(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonpath_in(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonpath_out(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonpath_recv(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn jsonpath_send(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn list_qsort(list: *const List, cmp: list_qsort_comparator) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn locate_var_of_level(
        node: *mut Node,
        levelsup: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn looks_like_temp_rel_name(name: *const ::std::os::raw::c_char) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn lookup_rowtype_tupdesc_domain(type_id: Oid, typmod: int32, noError: bool) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn makeArrayTypeName(
        typeName: *const ::std::os::raw::c_char,
        typeNamespace: Oid,
    ) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn makeIndexInfo(
        numattrs: ::std::os::raw::c_int,
        numkeyattrs: ::std::os::raw::c_int,
        amoid: Oid,
        expressions: *mut List,
        predicates: *mut List,
        unique: bool,
        isready: bool,
        concurrent: bool,
    ) -> *mut IndexInfo;
}
#[pg_guard]
extern "C" {
    pub fn makeInteger(i: ::std::os::raw::c_int) -> *mut Value;
}
#[pg_guard]
extern "C" {
    pub fn makeParamList(numParams: ::std::os::raw::c_int) -> ParamListInfo;
}
#[pg_guard]
extern "C" {
    pub fn makeVacuumRelation(
        relation: *mut RangeVar,
        oid: Oid,
        va_cols: *mut List,
    ) -> *mut VacuumRelation;
}
#[pg_guard]
extern "C" {
    pub fn mark_dummy_rel(rel: *mut RelOptInfo);
}
#[pg_guard]
extern "C" {
    pub fn minimal_expand_tuple(sourceTuple: HeapTuple, tupleDesc: TupleDesc) -> MinimalTuple;
}
#[pg_guard]
extern "C" {
    pub fn moveArrayTypeName(
        typeOid: Oid,
        typeName: *const ::std::os::raw::c_char,
        typeNamespace: Oid,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn nameconcatoid(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn namecpy(n1: Name, n2: *const NameData) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn nameeqtext(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn namegetext(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn namegttext(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn nameletext(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn namelttext(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn namenetext(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn negate_clause(node: *mut Node) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn network_subset_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn numeric_add_opt_error(num1: Numeric, num2: Numeric, have_error: *mut bool) -> Numeric;
}
#[pg_guard]
extern "C" {
    pub fn numeric_div_opt_error(num1: Numeric, num2: Numeric, have_error: *mut bool) -> Numeric;
}
#[pg_guard]
extern "C" {
    pub fn numeric_int4_opt_error(num: Numeric, error: *mut bool) -> int32;
}
#[pg_guard]
extern "C" {
    pub fn numeric_mod_opt_error(num1: Numeric, num2: Numeric, have_error: *mut bool) -> Numeric;
}
#[pg_guard]
extern "C" {
    pub fn numeric_mul_opt_error(num1: Numeric, num2: Numeric, have_error: *mut bool) -> Numeric;
}
#[pg_guard]
extern "C" {
    pub fn numeric_sub_opt_error(num1: Numeric, num2: Numeric, have_error: *mut bool) -> Numeric;
}
#[pg_guard]
extern "C" {
    pub fn numeric_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn parse_real(
        value: *const ::std::os::raw::c_char,
        result: *mut f64,
        flags: ::std::os::raw::c_int,
        hintmsg: *mut *const ::std::os::raw::c_char,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn pg_copy_logical_replication_slot_a(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_copy_logical_replication_slot_b(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_copy_logical_replication_slot_c(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_copy_physical_replication_slot_a(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_copy_physical_replication_slot_b(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_fprintf(
        stream: *mut FILE,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pg_indexam_progress_phasename(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_jit_available(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_ls_archive_statusdir(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_ls_tmpdir_1arg(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_ls_tmpdir_noargs(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_lsn_hash_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_mcv_list_in(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_mcv_list_out(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_mcv_list_recv(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_mcv_list_send(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_nextoid(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_partition_ancestors(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_partition_root(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_partition_tree(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_printf(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pg_promote(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_read_file_v2(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_replication_slot_advance(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_rotate_logfile_v2(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_snprintf(
        str: *mut ::std::os::raw::c_char,
        count: usize,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pg_sprintf(
        str: *mut ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pg_stat_get_db_checksum_failures(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_stat_get_db_checksum_last_failure(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_stats_ext_mcvlist_items(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn pg_strerror(errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn pg_strerror_r(
        errnum: ::std::os::raw::c_int,
        buf: *mut ::std::os::raw::c_char,
        buflen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn pg_strfromd(
        str: *mut ::std::os::raw::c_char,
        count: usize,
        precision: ::std::os::raw::c_int,
        value: f64,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pg_strsignal(signum: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn pg_strtoint16(s: *const ::std::os::raw::c_char) -> int16;
}
#[pg_guard]
extern "C" {
    pub fn pg_strtoint32(s: *const ::std::os::raw::c_char) -> int32;
}
#[pg_guard]
extern "C" {
    pub fn pg_vfprintf(
        stream: *mut FILE,
        fmt: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pg_vprintf(
        fmt: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pg_vsnprintf(
        str: *mut ::std::os::raw::c_char,
        count: usize,
        fmt: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pg_vsprintf(
        str: *mut ::std::os::raw::c_char,
        fmt: *const ::std::os::raw::c_char,
        args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn pgstat_clip_activity(
        raw_activity: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn pgstat_init_function_usage(fcinfo: FunctionCallInfo, fcu: *mut PgStat_FunctionCallUsage);
}
#[pg_guard]
extern "C" {
    pub fn pgstat_report_checksum_failure();
}
#[pg_guard]
extern "C" {
    pub fn pgstat_report_checksum_failures_in_db(dboid: Oid, failurecount: ::std::os::raw::c_int);
}
#[pg_guard]
extern "C" {
    pub fn plan_create_index_workers(tableOid: Oid, indexOid: Oid) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn planner(
        parse: *mut Query,
        cursorOptions: ::std::os::raw::c_int,
        boundParams: *mut ParamListInfoData,
    ) -> *mut PlannedStmt;
}
#[pg_guard]
extern "C" {
    pub fn predicate_implied_by(
        predicate_list: *mut List,
        clause_list: *mut List,
        weak: bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn predicate_refuted_by(
        predicate_list: *mut List,
        clause_list: *mut List,
        weak: bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn prefixjoinsel(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn prefixsel(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn process_equivalence(
        root: *mut PlannerInfo,
        p_restrictinfo: *mut *mut RestrictInfo,
        below_outer_join: bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn pull_var_clause(node: *mut Node, flags: ::std::os::raw::c_int) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn pull_varattnos(node: *mut Node, varno: Index, varattnos: *mut *mut Bitmapset);
}
#[pg_guard]
extern "C" {
    pub fn pull_varnos(node: *mut Node) -> *mut Bitmapset;
}
#[pg_guard]
extern "C" {
    pub fn pull_varnos_of_level(node: *mut Node, levelsup: ::std::os::raw::c_int)
        -> *mut Bitmapset;
}
#[pg_guard]
extern "C" {
    pub fn pull_vars_of_level(node: *mut Node, levelsup: ::std::os::raw::c_int) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn range_table_entry_walker(
        rte: *mut RangeTblEntry,
        walker: ::std::option::Option<unsafe extern "C" fn() -> bool>,
        context: *mut ::std::os::raw::c_void,
        flags: ::std::os::raw::c_int,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn reparameterize_path_by_child(
        root: *mut PlannerInfo,
        path: *mut Path,
        child_rel: *mut RelOptInfo,
    ) -> *mut Path;
}
#[pg_guard]
extern "C" {
    pub fn satisfies_hash_partition(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn scalargejoinsel(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn scalargesel(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn scalarlejoinsel(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn scalarlesel(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn set_result_size_estimates(root: *mut PlannerInfo, rel: *mut RelOptInfo);
}
#[pg_guard]
extern "C" {
    pub fn setup_append_rel_array(root: *mut PlannerInfo);
}
#[pg_guard]
extern "C" {
    pub fn sha224_bytea(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn sha256_bytea(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn sha384_bytea(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn sha512_bytea(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn simple_table_tuple_delete(rel: Relation, tid: ItemPointer, snapshot: Snapshot);
}
#[pg_guard]
extern "C" {
    pub fn simple_table_tuple_insert(rel: Relation, slot: *mut TupleTableSlot);
}
#[pg_guard]
extern "C" {
    pub fn simple_table_tuple_update(
        rel: Relation,
        otid: ItemPointer,
        slot: *mut TupleTableSlot,
        snapshot: Snapshot,
        update_indexes: *mut bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn slot_getmissingattrs(
        slot: *mut TupleTableSlot,
        startAttNum: ::std::os::raw::c_int,
        lastAttNum: ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn slot_getsomeattrs_int(slot: *mut TupleTableSlot, attnum: ::std::os::raw::c_int);
}
#[pg_guard]
extern "C" {
    pub fn sort_object_addresses(addrs: *mut ObjectAddresses);
}
#[pg_guard]
extern "C" {
    pub fn spg_bbox_quad_config(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn spg_poly_quad_compress(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn stringToNode(str: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void;
}
#[pg_guard]
extern "C" {
    pub fn sts_attach(
        sts: *mut SharedTuplestore,
        my_participant_number: ::std::os::raw::c_int,
        fileset: *mut SharedFileSet,
    ) -> *mut SharedTuplestoreAccessor;
}
#[pg_guard]
extern "C" {
    pub fn sts_begin_parallel_scan(accessor: *mut SharedTuplestoreAccessor);
}
#[pg_guard]
extern "C" {
    pub fn sts_end_parallel_scan(accessor: *mut SharedTuplestoreAccessor);
}
#[pg_guard]
extern "C" {
    pub fn sts_end_write(accessor: *mut SharedTuplestoreAccessor);
}
#[pg_guard]
extern "C" {
    pub fn sts_estimate(participants: ::std::os::raw::c_int) -> usize;
}
#[pg_guard]
extern "C" {
    pub fn sts_initialize(
        sts: *mut SharedTuplestore,
        participants: ::std::os::raw::c_int,
        my_participant_number: ::std::os::raw::c_int,
        meta_data_size: usize,
        flags: ::std::os::raw::c_int,
        fileset: *mut SharedFileSet,
        name: *const ::std::os::raw::c_char,
    ) -> *mut SharedTuplestoreAccessor;
}
#[pg_guard]
extern "C" {
    pub fn sts_parallel_scan_next(
        accessor: *mut SharedTuplestoreAccessor,
        meta_data: *mut ::std::os::raw::c_void,
    ) -> MinimalTuple;
}
#[pg_guard]
extern "C" {
    pub fn sts_puttuple(
        accessor: *mut SharedTuplestoreAccessor,
        meta_data: *mut ::std::os::raw::c_void,
        tuple: MinimalTuple,
    );
}
#[pg_guard]
extern "C" {
    pub fn sts_reinitialize(accessor: *mut SharedTuplestoreAccessor);
}
#[pg_guard]
extern "C" {
    pub fn table_am_handler_in(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn table_am_handler_out(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn table_beginscan_catalog(
        rel: Relation,
        nkeys: ::std::os::raw::c_int,
        key: *mut ScanKeyData,
    ) -> TableScanDesc;
}
#[pg_guard]
extern "C" {
    pub fn table_beginscan_parallel(rel: Relation, pscan: ParallelTableScanDesc) -> TableScanDesc;
}
#[pg_guard]
extern "C" {
    pub fn table_block_parallelscan_estimate(rel: Relation) -> Size;
}
#[pg_guard]
extern "C" {
    pub fn table_block_parallelscan_initialize(rel: Relation, pscan: ParallelTableScanDesc)
        -> Size;
}
#[pg_guard]
extern "C" {
    pub fn table_block_parallelscan_nextpage(
        rel: Relation,
        pbscan: ParallelBlockTableScanDesc,
    ) -> BlockNumber;
}
#[pg_guard]
extern "C" {
    pub fn table_block_parallelscan_reinitialize(rel: Relation, pscan: ParallelTableScanDesc);
}
#[pg_guard]
extern "C" {
    pub fn table_block_parallelscan_startblock_init(
        rel: Relation,
        pbscan: ParallelBlockTableScanDesc,
    );
}
#[pg_guard]
extern "C" {
    pub fn table_index_fetch_tuple_check(
        rel: Relation,
        tid: ItemPointer,
        snapshot: Snapshot,
        all_dead: *mut bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn table_parallelscan_estimate(rel: Relation, snapshot: Snapshot) -> Size;
}
#[pg_guard]
extern "C" {
    pub fn table_parallelscan_initialize(
        rel: Relation,
        pscan: ParallelTableScanDesc,
        snapshot: Snapshot,
    );
}
#[pg_guard]
extern "C" {
    pub fn table_scan_update_snapshot(scan: TableScanDesc, snapshot: Snapshot);
}
#[pg_guard]
extern "C" {
    pub fn table_slot_callbacks(rel: Relation) -> *const TupleTableSlotOps;
}
#[pg_guard]
extern "C" {
    pub fn table_slot_create(rel: Relation, reglist: *mut *mut List) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn table_tuple_get_latest_tid(scan: TableScanDesc, tid: ItemPointer);
}
#[pg_guard]
extern "C" {
    pub fn tbm_calculate_entries(maxbytes: f64) -> ::std::os::raw::c_long;
}
#[pg_guard]
extern "C" {
    pub fn text_starts_with(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn texteqname(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn textgename(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn textgtname(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn texticlike_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn texticregexeq_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn textlename(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn textlike_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn textltname(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn textnename(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn textregexeq_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn time2tm(time: TimeADT, tm: *mut pg_tm, fsec: *mut fsec_t) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn time_hash_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn time_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timestamp_hash_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timestamp_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timestamptz_trunc_zone(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timetz2tm(
        time: *mut TimeTzADT,
        tm: *mut pg_tm,
        fsec: *mut fsec_t,
        tzp: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn timetz_hash_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn transformContainerSubscripts(
        pstate: *mut ParseState,
        containerBase: *mut Node,
        containerType: Oid,
        elementType: Oid,
        containerTypMod: int32,
        indirection: *mut List,
        assignFrom: *mut Node,
    ) -> *mut SubscriptingRef;
}
#[pg_guard]
extern "C" {
    pub fn transformContainerType(containerType: *mut Oid, containerTypmod: *mut int32) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn transformRelOptions(
        oldOptions: Datum,
        defList: *mut List,
        namspace: *const ::std::os::raw::c_char,
        validnsps: *mut *mut ::std::os::raw::c_char,
        acceptOidsOff: bool,
        isReset: bool,
    ) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tuplehash_reset(tb: *mut tuplehash_hash);
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_attach_shared(shared: *mut Sharedsort, seg: *mut dsm_segment);
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_cluster(
        tupDesc: TupleDesc,
        indexRel: Relation,
        workMem: ::std::os::raw::c_int,
        coordinate: SortCoordinate,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_datum(
        datumType: Oid,
        sortOperator: Oid,
        sortCollation: Oid,
        nullsFirstFlag: bool,
        workMem: ::std::os::raw::c_int,
        coordinate: SortCoordinate,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_heap(
        tupDesc: TupleDesc,
        nkeys: ::std::os::raw::c_int,
        attNums: *mut AttrNumber,
        sortOperators: *mut Oid,
        sortCollations: *mut Oid,
        nullsFirstFlags: *mut bool,
        workMem: ::std::os::raw::c_int,
        coordinate: SortCoordinate,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_index_btree(
        heapRel: Relation,
        indexRel: Relation,
        enforceUnique: bool,
        workMem: ::std::os::raw::c_int,
        coordinate: SortCoordinate,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_index_hash(
        heapRel: Relation,
        indexRel: Relation,
        high_mask: uint32,
        low_mask: uint32,
        max_buckets: uint32,
        workMem: ::std::os::raw::c_int,
        coordinate: SortCoordinate,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_estimate_shared(nworkers: ::std::os::raw::c_int) -> Size;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_get_stats(state: *mut Tuplesortstate, stats: *mut TuplesortInstrumentation);
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_initialize_shared(
        shared: *mut Sharedsort,
        nWorkers: ::std::os::raw::c_int,
        seg: *mut dsm_segment,
    );
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_method_name(m: TuplesortMethod) -> *const ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_space_type_name(t: TuplesortSpaceType) -> *const ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn typeOrDomainTypeRelid(type_id: Oid) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn uuid_hash_extended(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn var_eq_const(
        vardata: *mut VariableStatData,
        oproid: Oid,
        constval: Datum,
        constisnull: bool,
        varonleft: bool,
        negate: bool,
    ) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn var_eq_non_const(
        vardata: *mut VariableStatData,
        oproid: Oid,
        other: *mut Node,
        varonleft: bool,
        negate: bool,
    ) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn varbit_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn varchar_support(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn websearch_to_tsquery(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn websearch_to_tsquery_byid(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub static TTSOpsBufferHeapTuple: TupleTableSlotOps;
}
#[pg_guard]
extern "C" {
    pub static TTSOpsHeapTuple: TupleTableSlotOps;
}
#[pg_guard]
extern "C" {
    pub static TTSOpsMinimalTuple: TupleTableSlotOps;
}
#[pg_guard]
extern "C" {
    pub static TTSOpsVirtual: TupleTableSlotOps;
}
#[pg_guard]
extern "C" {
    pub static mut AuxProcessResourceOwner: ResourceOwner;
}
#[pg_guard]
extern "C" {
    pub static mut CatalogSnapshotData: SnapshotData;
}
#[pg_guard]
extern "C" {
    pub static mut ClientConnectionLost: sig_atomic_t;
}
#[pg_guard]
extern "C" {
    pub static mut IdleInTransactionSessionTimeoutPending: sig_atomic_t;
}
#[pg_guard]
extern "C" {
    pub static mut InterruptPending: sig_atomic_t;
}
#[pg_guard]
extern "C" {
    pub static mut MainLWLockNames: [*const ::std::os::raw::c_char; 0usize];
}
#[pg_guard]
extern "C" {
    pub static mut MyStartTimestamp: TimestampTz;
}
#[pg_guard]
extern "C" {
    pub static mut PrimaryConnInfo: *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut PrimarySlotName: *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut ProcDiePending: sig_atomic_t;
}
#[pg_guard]
extern "C" {
    pub static mut PromoteTriggerFile: *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut QueryCancelPending: sig_atomic_t;
}
#[pg_guard]
extern "C" {
    pub static mut SnapshotAnyData: SnapshotData;
}
#[pg_guard]
extern "C" {
    pub static mut SnapshotSelfData: SnapshotData;
}
#[pg_guard]
extern "C" {
    pub static mut StandbyMode: bool;
}
#[pg_guard]
extern "C" {
    pub static mut StandbyModeRequested: bool;
}
#[pg_guard]
extern "C" {
    pub static mut VacuumCostDelay: f64;
}
#[pg_guard]
extern "C" {
    pub static mut archiveCleanupCommand: *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut data_directory_mode: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut default_table_access_method: *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut enable_parallel_append: bool;
}
#[pg_guard]
extern "C" {
    pub static mut enable_parallel_hash: bool;
}
#[pg_guard]
extern "C" {
    pub static mut enable_partition_pruning: bool;
}
#[pg_guard]
extern "C" {
    pub static mut enable_partitionwise_aggregate: bool;
}
#[pg_guard]
extern "C" {
    pub static mut enable_partitionwise_join: bool;
}
#[pg_guard]
extern "C" {
    pub static mut force_parallel_mode: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut log_xact_sample_rate: f64;
}
#[pg_guard]
extern "C" {
    pub static mut max_parallel_maintenance_workers: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut parallel_leader_participation: bool;
}
#[pg_guard]
extern "C" {
    pub static mut plan_cache_mode: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryEndCommand: *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryRestoreCommand: *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTarget: RecoveryTargetType;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTargetAction: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTargetInclusive: bool;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTargetLSN: XLogRecPtr;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTargetName: *const ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTargetTLI: TimeLineID;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTargetTLIRequested: TimeLineID;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTargetTimeLineGoal: RecoveryTargetTimeLineGoal;
}
#[pg_guard]
extern "C" {
    pub static mut recoveryTargetXid: TransactionId;
}
#[pg_guard]
extern "C" {
    pub static mut recovery_min_apply_delay: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut recovery_target_time_string: *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub static mut shmem_exit_inprogress: bool;
}
#[pg_guard]
extern "C" {
    pub static mut synchronize_seqscans: bool;
}
#[pg_guard]
extern "C" {
    pub static mut tcp_user_timeout: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut vacuum_cleanup_index_scale_factor: f64;
}
#[pg_guard]
extern "C" {
    pub static mut wal_init_zero: bool;
}
#[pg_guard]
extern "C" {
    pub static mut wal_recycle: bool;
}
#[pg_guard]
extern "C" {
    pub static mut wal_segment_size: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut xact_is_sampled: bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BackgroundWorker {
    pub bgw_name: [::std::os::raw::c_char; 96usize],
    pub bgw_type: [::std::os::raw::c_char; 96usize],
    pub bgw_flags: ::std::os::raw::c_int,
    pub bgw_start_time: BgWorkerStartTime,
    pub bgw_restart_time: ::std::os::raw::c_int,
    pub bgw_library_name: [::std::os::raw::c_char; 96usize],
    pub bgw_function_name: [::std::os::raw::c_char; 96usize],
    pub bgw_main_arg: Datum,
    pub bgw_extra: [::std::os::raw::c_char; 128usize],
    pub bgw_notify_pid: pid_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_attribute {
    pub attrelid: Oid,
    pub attname: NameData,
    pub atttypid: Oid,
    pub attstattarget: int32,
    pub attlen: int16,
    pub attnum: int16,
    pub attndims: int32,
    pub attcacheoff: int32,
    pub atttypmod: int32,
    pub attbyval: bool,
    pub attstorage: ::std::os::raw::c_char,
    pub attalign: ::std::os::raw::c_char,
    pub attnotnull: bool,
    pub atthasdef: bool,
    pub atthasmissing: bool,
    pub attidentity: ::std::os::raw::c_char,
    pub attgenerated: ::std::os::raw::c_char,
    pub attisdropped: bool,
    pub attislocal: bool,
    pub attinhcount: int32,
    pub attcollation: Oid,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_class {
    pub oid: Oid,
    pub relname: NameData,
    pub relnamespace: Oid,
    pub reltype: Oid,
    pub reloftype: Oid,
    pub relowner: Oid,
    pub relam: Oid,
    pub relfilenode: Oid,
    pub reltablespace: Oid,
    pub relpages: int32,
    pub reltuples: float4,
    pub relallvisible: int32,
    pub reltoastrelid: Oid,
    pub relhasindex: bool,
    pub relisshared: bool,
    pub relpersistence: ::std::os::raw::c_char,
    pub relkind: ::std::os::raw::c_char,
    pub relnatts: int16,
    pub relchecks: int16,
    pub relhasrules: bool,
    pub relhastriggers: bool,
    pub relhassubclass: bool,
    pub relrowsecurity: bool,
    pub relforcerowsecurity: bool,
    pub relispopulated: bool,
    pub relreplident: ::std::os::raw::c_char,
    pub relispartition: bool,
    pub relrewrite: Oid,
    pub relfrozenxid: TransactionId,
    pub relminmxid: TransactionId,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_enum {
    pub oid: Oid,
    pub enumtypid: Oid,
    pub enumsortorder: float4,
    pub enumlabel: NameData,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_event_trigger {
    pub oid: Oid,
    pub evtname: NameData,
    pub evtevent: NameData,
    pub evtowner: Oid,
    pub evtfoid: Oid,
    pub evtenabled: ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_publication {
    pub oid: Oid,
    pub pubname: NameData,
    pub pubowner: Oid,
    pub puballtables: bool,
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
    pub pubtruncate: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_type {
    pub oid: Oid,
    pub typname: NameData,
    pub typnamespace: Oid,
    pub typowner: Oid,
    pub typlen: int16,
    pub typbyval: bool,
    pub typtype: ::std::os::raw::c_char,
    pub typcategory: ::std::os::raw::c_char,
    pub typispreferred: bool,
    pub typisdefined: bool,
    pub typdelim: ::std::os::raw::c_char,
    pub typrelid: Oid,
    pub typelem: Oid,
    pub typarray: Oid,
    pub typinput: regproc,
    pub typoutput: regproc,
    pub typreceive: regproc,
    pub typsend: regproc,
    pub typmodin: regproc,
    pub typmodout: regproc,
    pub typanalyze: regproc,
    pub typalign: ::std::os::raw::c_char,
    pub typstorage: ::std::os::raw::c_char,
    pub typnotnull: bool,
    pub typbasetype: Oid,
    pub typtypmod: int32,
    pub typndims: int32,
    pub typcollation: Oid,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PGPROC {
    pub links: SHM_QUEUE,
    pub procgloballist: *mut *mut PGPROC,
    pub sem: PGSemaphore,
    pub waitStatus: ::std::os::raw::c_int,
    pub procLatch: Latch,
    pub lxid: LocalTransactionId,
    pub pid: ::std::os::raw::c_int,
    pub pgprocno: ::std::os::raw::c_int,
    pub backendId: BackendId,
    pub databaseId: Oid,
    pub roleId: Oid,
    pub tempNamespaceId: Oid,
    pub isBackgroundWorker: bool,
    pub recoveryConflictPending: bool,
    pub lwWaiting: bool,
    pub lwWaitMode: uint8,
    pub lwWaitLink: proclist_node,
    pub cvWaitLink: proclist_node,
    pub waitLock: *mut LOCK,
    pub waitProcLock: *mut PROCLOCK,
    pub waitLockMode: LOCKMODE,
    pub heldLocks: LOCKMASK,
    pub waitLSN: XLogRecPtr,
    pub syncRepState: ::std::os::raw::c_int,
    pub syncRepLinks: SHM_QUEUE,
    pub myProcLocks: [SHM_QUEUE; 16usize],
    pub subxids: XidCache,
    pub procArrayGroupMember: bool,
    pub procArrayGroupNext: pg_atomic_uint32,
    pub procArrayGroupMemberXid: TransactionId,
    pub wait_event_info: uint32,
    pub clogGroupMember: bool,
    pub clogGroupNext: pg_atomic_uint32,
    pub clogGroupMemberXid: TransactionId,
    pub clogGroupMemberXidStatus: XidStatus,
    pub clogGroupMemberPage: ::std::os::raw::c_int,
    pub clogGroupMemberLsn: XLogRecPtr,
    pub backendLock: LWLock,
    pub fpLockBits: uint64,
    pub fpRelId: [Oid; 16usize],
    pub fpVXIDLock: bool,
    pub fpLocalTransactionId: LocalTransactionId,
    pub lockGroupLeader: *mut PGPROC,
    pub lockGroupMembers: dlist_head,
    pub lockGroupLink: dlist_node,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgBackendGSSStatus {
    pub gss_princ: [::std::os::raw::c_char; 64usize],
    pub gss_auth: bool,
    pub gss_enc: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgBackendSSLStatus {
    pub ssl_bits: ::std::os::raw::c_int,
    pub ssl_compression: bool,
    pub ssl_version: [::std::os::raw::c_char; 64usize],
    pub ssl_cipher: [::std::os::raw::c_char; 64usize],
    pub ssl_client_dn: [::std::os::raw::c_char; 64usize],
    pub ssl_client_serial: [::std::os::raw::c_char; 64usize],
    pub ssl_issuer_dn: [::std::os::raw::c_char; 64usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgBackendStatus {
    pub st_changecount: ::std::os::raw::c_int,
    pub st_procpid: ::std::os::raw::c_int,
    pub st_backendType: BackendType,
    pub st_proc_start_timestamp: TimestampTz,
    pub st_xact_start_timestamp: TimestampTz,
    pub st_activity_start_timestamp: TimestampTz,
    pub st_state_start_timestamp: TimestampTz,
    pub st_databaseid: Oid,
    pub st_userid: Oid,
    pub st_clientaddr: SockAddr,
    pub st_clienthostname: *mut ::std::os::raw::c_char,
    pub st_ssl: bool,
    pub st_sslstatus: *mut PgBackendSSLStatus,
    pub st_gss: bool,
    pub st_gssstatus: *mut PgBackendGSSStatus,
    pub st_state: BackendState,
    pub st_appname: *mut ::std::os::raw::c_char,
    pub st_activity_raw: *mut ::std::os::raw::c_char,
    pub st_progress_command: ProgressCommandType,
    pub st_progress_command_target: Oid,
    pub st_progress_param: [int64; 20usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XLogReaderState {
    pub wal_segment_size: ::std::os::raw::c_int,
    pub read_page: XLogPageReadCB,
    pub system_identifier: uint64,
    pub private_data: *mut ::std::os::raw::c_void,
    pub ReadRecPtr: XLogRecPtr,
    pub EndRecPtr: XLogRecPtr,
    pub decoded_record: *mut XLogRecord,
    pub main_data: *mut ::std::os::raw::c_char,
    pub main_data_len: uint32,
    pub main_data_bufsz: uint32,
    pub record_origin: RepOriginId,
    pub blocks: [DecodedBkpBlock; 33usize],
    pub max_block_id: ::std::os::raw::c_int,
    pub readBuf: *mut ::std::os::raw::c_char,
    pub readLen: uint32,
    pub readSegNo: XLogSegNo,
    pub readOff: uint32,
    pub readPageTLI: TimeLineID,
    pub latestPagePtr: XLogRecPtr,
    pub latestPageTLI: TimeLineID,
    pub currRecPtr: XLogRecPtr,
    pub currTLI: TimeLineID,
    pub currTLIValidUntil: XLogRecPtr,
    pub nextTLI: TimeLineID,
    pub readRecordBuf: *mut ::std::os::raw::c_char,
    pub readRecordBufSize: uint32,
    pub errormsg_buf: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xl_xact_parsed_abort {
    pub xact_time: TimestampTz,
    pub xinfo: uint32,
    pub dbId: Oid,
    pub tsId: Oid,
    pub nsubxacts: ::std::os::raw::c_int,
    pub subxacts: *mut TransactionId,
    pub nrels: ::std::os::raw::c_int,
    pub xnodes: *mut RelFileNode,
    pub twophase_xid: TransactionId,
    pub twophase_gid: [::std::os::raw::c_char; 200usize],
    pub origin_lsn: XLogRecPtr,
    pub origin_timestamp: TimestampTz,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xl_xact_parsed_commit {
    pub xact_time: TimestampTz,
    pub xinfo: uint32,
    pub dbId: Oid,
    pub tsId: Oid,
    pub nsubxacts: ::std::os::raw::c_int,
    pub subxacts: *mut TransactionId,
    pub nrels: ::std::os::raw::c_int,
    pub xnodes: *mut RelFileNode,
    pub nmsgs: ::std::os::raw::c_int,
    pub msgs: *mut SharedInvalidationMessage,
    pub twophase_xid: TransactionId,
    pub twophase_gid: [::std::os::raw::c_char; 200usize],
    pub nabortrels: ::std::os::raw::c_int,
    pub abortnodes: *mut RelFileNode,
    pub origin_lsn: XLogRecPtr,
    pub origin_timestamp: TimestampTz,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PgStat_Msg {
    pub msg_hdr: PgStat_MsgHdr,
    pub msg_dummy: PgStat_MsgDummy,
    pub msg_inquiry: PgStat_MsgInquiry,
    pub msg_tabstat: PgStat_MsgTabstat,
    pub msg_tabpurge: PgStat_MsgTabpurge,
    pub msg_dropdb: PgStat_MsgDropdb,
    pub msg_resetcounter: PgStat_MsgResetcounter,
    pub msg_resetsharedcounter: PgStat_MsgResetsharedcounter,
    pub msg_resetsinglecounter: PgStat_MsgResetsinglecounter,
    pub msg_autovacuum_start: PgStat_MsgAutovacStart,
    pub msg_vacuum: PgStat_MsgVacuum,
    pub msg_analyze: PgStat_MsgAnalyze,
    pub msg_archiver: PgStat_MsgArchiver,
    pub msg_bgwriter: PgStat_MsgBgWriter,
    pub msg_funcstat: PgStat_MsgFuncstat,
    pub msg_funcpurge: PgStat_MsgFuncpurge,
    pub msg_recoveryconflict: PgStat_MsgRecoveryConflict,
    pub msg_deadlock: PgStat_MsgDeadlock,
    pub msg_tempfile: PgStat_MsgTempFile,
    pub msg_checksumfailure: PgStat_MsgChecksumFailure,
    _bindgen_union_align: [u64; 125usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Value_ValUnion {
    pub ival: ::std::os::raw::c_int,
    pub str: *mut ::std::os::raw::c_char,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug)]
pub struct FunctionCallInfoBaseData {
    pub flinfo: *mut FmgrInfo,
    pub context: fmNodePtr,
    pub resultinfo: fmNodePtr,
    pub fncollation: Oid,
    pub isnull: bool,
    pub nargs: ::std::os::raw::c_short,
    pub args: __IncompleteArrayField<NullableDatum>,
}
#[repr(C)]
#[derive(Debug)]
pub struct ParamListInfoData {
    pub paramFetch: ParamFetchHook,
    pub paramFetchArg: *mut ::std::os::raw::c_void,
    pub paramCompile: ParamCompileHook,
    pub paramCompileArg: *mut ::std::os::raw::c_void,
    pub parserSetup: ParserSetupHook,
    pub parserSetupArg: *mut ::std::os::raw::c_void,
    pub numParams: ::std::os::raw::c_int,
    pub params: __IncompleteArrayField<ParamExternData>,
}
#[repr(C)]
#[derive(Debug)]
pub struct SharedSortInfo {
    pub num_workers: ::std::os::raw::c_int,
    pub sinstrument: __IncompleteArrayField<TuplesortInstrumentation>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Agg {
    pub plan: Plan,
    pub aggstrategy: AggStrategy,
    pub aggsplit: AggSplit,
    pub numCols: ::std::os::raw::c_int,
    pub grpColIdx: *mut AttrNumber,
    pub grpOperators: *mut Oid,
    pub grpCollations: *mut Oid,
    pub numGroups: ::std::os::raw::c_long,
    pub aggParams: *mut Bitmapset,
    pub groupingSets: *mut List,
    pub chain: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AggState {
    pub ss: ScanState,
    pub aggs: *mut List,
    pub numaggs: ::std::os::raw::c_int,
    pub numtrans: ::std::os::raw::c_int,
    pub aggstrategy: AggStrategy,
    pub aggsplit: AggSplit,
    pub phase: AggStatePerPhase,
    pub numphases: ::std::os::raw::c_int,
    pub current_phase: ::std::os::raw::c_int,
    pub peragg: AggStatePerAgg,
    pub pertrans: AggStatePerTrans,
    pub hashcontext: *mut ExprContext,
    pub aggcontexts: *mut *mut ExprContext,
    pub tmpcontext: *mut ExprContext,
    pub curaggcontext: *mut ExprContext,
    pub curperagg: AggStatePerAgg,
    pub curpertrans: AggStatePerTrans,
    pub input_done: bool,
    pub agg_done: bool,
    pub projected_set: ::std::os::raw::c_int,
    pub current_set: ::std::os::raw::c_int,
    pub grouped_cols: *mut Bitmapset,
    pub all_grouped_cols: *mut List,
    pub maxsets: ::std::os::raw::c_int,
    pub phases: AggStatePerPhase,
    pub sort_in: *mut Tuplesortstate,
    pub sort_out: *mut Tuplesortstate,
    pub sort_slot: *mut TupleTableSlot,
    pub pergroups: *mut AggStatePerGroup,
    pub grp_firstTuple: HeapTuple,
    pub table_filled: bool,
    pub num_hashes: ::std::os::raw::c_int,
    pub perhash: AggStatePerHash,
    pub hash_pergroup: *mut AggStatePerGroup,
    pub all_pergroups: *mut AggStatePerGroup,
    pub combinedproj: *mut ProjectionInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlterFunctionStmt {
    pub type_: NodeTag,
    pub objtype: ObjectType,
    pub func: *mut ObjectWithArgs,
    pub actions: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlterTableCmd {
    pub type_: NodeTag,
    pub subtype: AlterTableType,
    pub name: *mut ::std::os::raw::c_char,
    pub num: int16,
    pub newowner: *mut RoleSpec,
    pub def: *mut Node,
    pub behavior: DropBehavior,
    pub missing_ok: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Append {
    pub plan: Plan,
    pub appendplans: *mut List,
    pub first_partial_plan: ::std::os::raw::c_int,
    pub part_prune_info: *mut PartitionPruneInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AppendPath {
    pub path: Path,
    pub partitioned_rels: *mut List,
    pub subpaths: *mut List,
    pub first_partial_path: ::std::os::raw::c_int,
    pub limit_tuples: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AppendState {
    pub ps: PlanState,
    pub appendplans: *mut *mut PlanState,
    pub as_nplans: ::std::os::raw::c_int,
    pub as_whichplan: ::std::os::raw::c_int,
    pub as_first_partial_plan: ::std::os::raw::c_int,
    pub as_pstate: *mut ParallelAppendState,
    pub pstate_len: Size,
    pub as_prune_state: *mut PartitionPruneState,
    pub as_valid_subplans: *mut Bitmapset,
    pub choose_next_subplan:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut AppendState) -> bool>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArrayCoerceExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub elemexpr: *mut Expr,
    pub resulttype: Oid,
    pub resulttypmod: int32,
    pub resultcollid: Oid,
    pub coerceformat: CoercionForm,
    pub location: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AttStatsSlot {
    pub staop: Oid,
    pub stacoll: Oid,
    pub valuetype: Oid,
    pub values: *mut Datum,
    pub nvalues: ::std::os::raw::c_int,
    pub numbers: *mut float4,
    pub nnumbers: ::std::os::raw::c_int,
    pub values_arr: *mut ::std::os::raw::c_void,
    pub numbers_arr: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AttrDefault {
    pub adnum: AttrNumber,
    pub adbin: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitmapHeapScanState {
    pub ss: ScanState,
    pub bitmapqualorig: *mut ExprState,
    pub tbm: *mut TIDBitmap,
    pub tbmiterator: *mut TBMIterator,
    pub tbmres: *mut TBMIterateResult,
    pub can_skip_fetch: bool,
    pub return_empty_tuples: ::std::os::raw::c_int,
    pub vmbuffer: Buffer,
    pub pvmbuffer: Buffer,
    pub exact_pages: ::std::os::raw::c_long,
    pub lossy_pages: ::std::os::raw::c_long,
    pub prefetch_iterator: *mut TBMIterator,
    pub prefetch_pages: ::std::os::raw::c_int,
    pub prefetch_target: ::std::os::raw::c_int,
    pub prefetch_maximum: ::std::os::raw::c_int,
    pub pscan_len: Size,
    pub initialized: bool,
    pub shared_tbmiterator: *mut TBMSharedIterator,
    pub shared_prefetch_iterator: *mut TBMSharedIterator,
    pub pstate: *mut ParallelBitmapHeapState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitmapIndexScanState {
    pub ss: ScanState,
    pub biss_result: *mut TIDBitmap,
    pub biss_ScanKeys: *mut ScanKeyData,
    pub biss_NumScanKeys: ::std::os::raw::c_int,
    pub biss_RuntimeKeys: *mut IndexRuntimeKeyInfo,
    pub biss_NumRuntimeKeys: ::std::os::raw::c_int,
    pub biss_ArrayKeys: *mut IndexArrayKeyInfo,
    pub biss_NumArrayKeys: ::std::os::raw::c_int,
    pub biss_RuntimeKeysReady: bool,
    pub biss_RuntimeContext: *mut ExprContext,
    pub biss_RelationDesc: Relation,
    pub biss_ScanDesc: *mut IndexScanDescData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BufferHeapTupleTableSlot {
    pub base: HeapTupleTableSlot,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CachedExpression {
    pub magic: ::std::os::raw::c_int,
    pub expr: *mut Node,
    pub is_valid: bool,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub context: MemoryContext,
    pub node: dlist_node,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CachedPlanSource {
    pub magic: ::std::os::raw::c_int,
    pub raw_parse_tree: *mut RawStmt,
    pub query_string: *const ::std::os::raw::c_char,
    pub commandTag: *const ::std::os::raw::c_char,
    pub param_types: *mut Oid,
    pub num_params: ::std::os::raw::c_int,
    pub parserSetup: ParserSetupHook,
    pub parserSetupArg: *mut ::std::os::raw::c_void,
    pub cursor_options: ::std::os::raw::c_int,
    pub fixed_result: bool,
    pub resultDesc: TupleDesc,
    pub context: MemoryContext,
    pub query_list: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub search_path: *mut OverrideSearchPath,
    pub query_context: MemoryContext,
    pub rewriteRoleId: Oid,
    pub rewriteRowSecurity: bool,
    pub dependsOnRLS: bool,
    pub gplan: *mut CachedPlan,
    pub is_oneshot: bool,
    pub is_complete: bool,
    pub is_saved: bool,
    pub is_valid: bool,
    pub generation: ::std::os::raw::c_int,
    pub node: dlist_node,
    pub generic_cost: f64,
    pub total_custom_cost: f64,
    pub num_custom_plans: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CallContext {
    pub type_: NodeTag,
    pub atomic: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CallStmt {
    pub type_: NodeTag,
    pub funccall: *mut FuncCall,
    pub funcexpr: *mut FuncExpr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClusterStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    pub indexname: *mut ::std::os::raw::c_char,
    pub options: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CollectedCommand__bindgen_ty_1__bindgen_ty_7 {
    pub objtype: ObjectType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColumnDef {
    pub type_: NodeTag,
    pub colname: *mut ::std::os::raw::c_char,
    pub typeName: *mut TypeName,
    pub inhcount: ::std::os::raw::c_int,
    pub is_local: bool,
    pub is_not_null: bool,
    pub is_from_type: bool,
    pub storage: ::std::os::raw::c_char,
    pub raw_default: *mut Node,
    pub cooked_default: *mut Node,
    pub identity: ::std::os::raw::c_char,
    pub identitySequence: *mut RangeVar,
    pub generated: ::std::os::raw::c_char,
    pub collClause: *mut CollateClause,
    pub collOid: Oid,
    pub constraints: *mut List,
    pub fdwoptions: *mut List,
    pub location: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommonTableExpr {
    pub type_: NodeTag,
    pub ctename: *mut ::std::os::raw::c_char,
    pub aliascolnames: *mut List,
    pub ctematerialized: CTEMaterialize,
    pub ctequery: *mut Node,
    pub location: ::std::os::raw::c_int,
    pub cterecursive: bool,
    pub cterefcount: ::std::os::raw::c_int,
    pub ctecolnames: *mut List,
    pub ctecoltypes: *mut List,
    pub ctecoltypmods: *mut List,
    pub ctecolcollations: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConstrCheck {
    pub ccname: *mut ::std::os::raw::c_char,
    pub ccbin: *mut ::std::os::raw::c_char,
    pub ccvalid: bool,
    pub ccnoinherit: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Constraint {
    pub type_: NodeTag,
    pub contype: ConstrType,
    pub conname: *mut ::std::os::raw::c_char,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub location: ::std::os::raw::c_int,
    pub is_no_inherit: bool,
    pub raw_expr: *mut Node,
    pub cooked_expr: *mut ::std::os::raw::c_char,
    pub generated_when: ::std::os::raw::c_char,
    pub keys: *mut List,
    pub including: *mut List,
    pub exclusions: *mut List,
    pub options: *mut List,
    pub indexname: *mut ::std::os::raw::c_char,
    pub indexspace: *mut ::std::os::raw::c_char,
    pub reset_default_tblspc: bool,
    pub access_method: *mut ::std::os::raw::c_char,
    pub where_clause: *mut Node,
    pub pktable: *mut RangeVar,
    pub fk_attrs: *mut List,
    pub pk_attrs: *mut List,
    pub fk_matchtype: ::std::os::raw::c_char,
    pub fk_upd_action: ::std::os::raw::c_char,
    pub fk_del_action: ::std::os::raw::c_char,
    pub old_conpfeqop: *mut List,
    pub old_pktable_oid: Oid,
    pub skip_validation: bool,
    pub initially_valid: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CopyMultiInsertBuffer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CopyStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    pub query: *mut Node,
    pub attlist: *mut List,
    pub is_from: bool,
    pub is_program: bool,
    pub filename: *mut ::std::os::raw::c_char,
    pub options: *mut List,
    pub whereClause: *mut Node,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CreateFunctionStmt {
    pub type_: NodeTag,
    pub is_procedure: bool,
    pub replace: bool,
    pub funcname: *mut List,
    pub parameters: *mut List,
    pub returnType: *mut TypeName,
    pub options: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CreateStatsStmt {
    pub type_: NodeTag,
    pub defnames: *mut List,
    pub stat_types: *mut List,
    pub exprs: *mut List,
    pub relations: *mut List,
    pub stxcomment: *mut ::std::os::raw::c_char,
    pub if_not_exists: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CreateStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    pub tableElts: *mut List,
    pub inhRelations: *mut List,
    pub partbound: *mut PartitionBoundSpec,
    pub partspec: *mut PartitionSpec,
    pub ofTypename: *mut TypeName,
    pub constraints: *mut List,
    pub options: *mut List,
    pub oncommit: OnCommitAction,
    pub tablespacename: *mut ::std::os::raw::c_char,
    pub accessMethod: *mut ::std::os::raw::c_char,
    pub if_not_exists: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CustomPathMethods {
    pub CustomName: *const ::std::os::raw::c_char,
    pub PlanCustomPath: ::std::option::Option<
        unsafe extern "C" fn(
            root: *mut PlannerInfo,
            rel: *mut RelOptInfo,
            best_path: *mut CustomPath,
            tlist: *mut List,
            clauses: *mut List,
            custom_plans: *mut List,
        ) -> *mut Plan,
    >,
    pub ReparameterizeCustomPathByChild: ::std::option::Option<
        unsafe extern "C" fn(
            root: *mut PlannerInfo,
            custom_private: *mut List,
            child_rel: *mut RelOptInfo,
        ) -> *mut List,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefineStmt {
    pub type_: NodeTag,
    pub kind: ObjectType,
    pub oldstyle: bool,
    pub defnames: *mut List,
    pub args: *mut List,
    pub definition: *mut List,
    pub if_not_exists: bool,
    pub replace: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EPQState {
    pub parentestate: *mut EState,
    pub epqParam: ::std::os::raw::c_int,
    pub tuple_table: *mut List,
    pub relsubs_slot: *mut *mut TupleTableSlot,
    pub plan: *mut Plan,
    pub arowMarks: *mut List,
    pub origslot: *mut TupleTableSlot,
    pub recheckestate: *mut EState,
    pub relsubs_rowmark: *mut *mut ExecAuxRowMark,
    pub relsubs_done: *mut bool,
    pub recheckplanstate: *mut PlanState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EState {
    pub type_: NodeTag,
    pub es_direction: ScanDirection,
    pub es_snapshot: Snapshot,
    pub es_crosscheck_snapshot: Snapshot,
    pub es_range_table: *mut List,
    pub es_range_table_array: *mut *mut RangeTblEntry,
    pub es_range_table_size: Index,
    pub es_relations: *mut Relation,
    pub es_rowmarks: *mut *mut ExecRowMark,
    pub es_plannedstmt: *mut PlannedStmt,
    pub es_sourceText: *const ::std::os::raw::c_char,
    pub es_junkFilter: *mut JunkFilter,
    pub es_output_cid: CommandId,
    pub es_result_relations: *mut ResultRelInfo,
    pub es_num_result_relations: ::std::os::raw::c_int,
    pub es_result_relation_info: *mut ResultRelInfo,
    pub es_root_result_relations: *mut ResultRelInfo,
    pub es_num_root_result_relations: ::std::os::raw::c_int,
    pub es_partition_directory: PartitionDirectory,
    pub es_tuple_routing_result_relations: *mut List,
    pub es_trig_target_relations: *mut List,
    pub es_param_list_info: ParamListInfo,
    pub es_param_exec_vals: *mut ParamExecData,
    pub es_queryEnv: *mut QueryEnvironment,
    pub es_query_cxt: MemoryContext,
    pub es_tupleTable: *mut List,
    pub es_processed: uint64,
    pub es_top_eflags: ::std::os::raw::c_int,
    pub es_instrument: ::std::os::raw::c_int,
    pub es_finished: bool,
    pub es_exprcontexts: *mut List,
    pub es_subplanstates: *mut List,
    pub es_auxmodifytables: *mut List,
    pub es_per_tuple_exprcontext: *mut ExprContext,
    pub es_epq_active: *mut EPQState,
    pub es_use_parallel_mode: bool,
    pub es_query_dsa: *mut dsa_area,
    pub es_jit_flags: ::std::os::raw::c_int,
    pub es_jit: *mut JitContext,
    pub es_jit_worker_instr: *mut JitInstrumentation,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExplainState {
    pub str: StringInfo,
    pub verbose: bool,
    pub analyze: bool,
    pub costs: bool,
    pub buffers: bool,
    pub timing: bool,
    pub summary: bool,
    pub settings: bool,
    pub format: ExplainFormat,
    pub indent: ::std::os::raw::c_int,
    pub grouping_stack: *mut List,
    pub pstmt: *mut PlannedStmt,
    pub rtable: *mut List,
    pub rtable_names: *mut List,
    pub deparse_cxt: *mut List,
    pub printed_subplans: *mut Bitmapset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExprState {
    pub tag: Node,
    pub flags: uint8,
    pub resnull: bool,
    pub resvalue: Datum,
    pub resultslot: *mut TupleTableSlot,
    pub steps: *mut ExprEvalStep,
    pub evalfunc: ExprStateEvalFunc,
    pub expr: *mut Expr,
    pub evalfunc_private: *mut ::std::os::raw::c_void,
    pub steps_len: ::std::os::raw::c_int,
    pub steps_alloc: ::std::os::raw::c_int,
    pub parent: *mut PlanState,
    pub ext_params: ParamListInfo,
    pub innermost_caseval: *mut Datum,
    pub innermost_casenull: *mut bool,
    pub innermost_domainval: *mut Datum,
    pub innermost_domainnull: *mut bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FdwRoutine {
    pub type_: NodeTag,
    pub GetForeignRelSize: GetForeignRelSize_function,
    pub GetForeignPaths: GetForeignPaths_function,
    pub GetForeignPlan: GetForeignPlan_function,
    pub BeginForeignScan: BeginForeignScan_function,
    pub IterateForeignScan: IterateForeignScan_function,
    pub ReScanForeignScan: ReScanForeignScan_function,
    pub EndForeignScan: EndForeignScan_function,
    pub GetForeignJoinPaths: GetForeignJoinPaths_function,
    pub GetForeignUpperPaths: GetForeignUpperPaths_function,
    pub AddForeignUpdateTargets: AddForeignUpdateTargets_function,
    pub PlanForeignModify: PlanForeignModify_function,
    pub BeginForeignModify: BeginForeignModify_function,
    pub ExecForeignInsert: ExecForeignInsert_function,
    pub ExecForeignUpdate: ExecForeignUpdate_function,
    pub ExecForeignDelete: ExecForeignDelete_function,
    pub EndForeignModify: EndForeignModify_function,
    pub BeginForeignInsert: BeginForeignInsert_function,
    pub EndForeignInsert: EndForeignInsert_function,
    pub IsForeignRelUpdatable: IsForeignRelUpdatable_function,
    pub PlanDirectModify: PlanDirectModify_function,
    pub BeginDirectModify: BeginDirectModify_function,
    pub IterateDirectModify: IterateDirectModify_function,
    pub EndDirectModify: EndDirectModify_function,
    pub GetForeignRowMarkType: GetForeignRowMarkType_function,
    pub RefetchForeignRow: RefetchForeignRow_function,
    pub RecheckForeignScan: RecheckForeignScan_function,
    pub ExplainForeignScan: ExplainForeignScan_function,
    pub ExplainForeignModify: ExplainForeignModify_function,
    pub ExplainDirectModify: ExplainDirectModify_function,
    pub AnalyzeForeignTable: AnalyzeForeignTable_function,
    pub ImportForeignSchema: ImportForeignSchema_function,
    pub IsForeignScanParallelSafe: IsForeignScanParallelSafe_function,
    pub EstimateDSMForeignScan: EstimateDSMForeignScan_function,
    pub InitializeDSMForeignScan: InitializeDSMForeignScan_function,
    pub ReInitializeDSMForeignScan: ReInitializeDSMForeignScan_function,
    pub InitializeWorkerForeignScan: InitializeWorkerForeignScan_function,
    pub ShutdownForeignScan: ShutdownForeignScan_function,
    pub ReparameterizeForeignPathByChild: ReparameterizeForeignPathByChild_function,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ForeignKeyCacheInfo {
    pub type_: NodeTag,
    pub conoid: Oid,
    pub conrelid: Oid,
    pub confrelid: Oid,
    pub nkeys: ::std::os::raw::c_int,
    pub conkey: [AttrNumber; 32usize],
    pub confkey: [AttrNumber; 32usize],
    pub conpfeqop: [Oid; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FuncCallContext {
    pub call_cntr: uint64,
    pub max_calls: uint64,
    pub user_fctx: *mut ::std::os::raw::c_void,
    pub attinmeta: *mut AttInMetadata,
    pub multi_call_memory_ctx: MemoryContext,
    pub tuple_desc: TupleDesc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gather {
    pub plan: Plan,
    pub num_workers: ::std::os::raw::c_int,
    pub rescan_param: ::std::os::raw::c_int,
    pub single_copy: bool,
    pub invisible: bool,
    pub initParam: *mut Bitmapset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GatherMerge {
    pub plan: Plan,
    pub num_workers: ::std::os::raw::c_int,
    pub rescan_param: ::std::os::raw::c_int,
    pub numCols: ::std::os::raw::c_int,
    pub sortColIdx: *mut AttrNumber,
    pub sortOperators: *mut Oid,
    pub collations: *mut Oid,
    pub nullsFirst: *mut bool,
    pub initParam: *mut Bitmapset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GatherMergeState {
    pub ps: PlanState,
    pub initialized: bool,
    pub gm_initialized: bool,
    pub need_to_scan_locally: bool,
    pub tuples_needed: int64,
    pub tupDesc: TupleDesc,
    pub gm_nkeys: ::std::os::raw::c_int,
    pub gm_sortkeys: SortSupport,
    pub pei: *mut ParallelExecutorInfo,
    pub nworkers_launched: ::std::os::raw::c_int,
    pub nreaders: ::std::os::raw::c_int,
    pub gm_slots: *mut *mut TupleTableSlot,
    pub reader: *mut *mut TupleQueueReader,
    pub gm_tuple_buffers: *mut GMReaderTupleBuffer,
    pub gm_heap: *mut binaryheap,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GatherState {
    pub ps: PlanState,
    pub initialized: bool,
    pub need_to_scan_locally: bool,
    pub tuples_needed: int64,
    pub funnel_slot: *mut TupleTableSlot,
    pub pei: *mut ParallelExecutorInfo,
    pub nworkers_launched: ::std::os::raw::c_int,
    pub nreaders: ::std::os::raw::c_int,
    pub nextreader: ::std::os::raw::c_int,
    pub reader: *mut *mut TupleQueueReader,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GrantStmt {
    pub type_: NodeTag,
    pub is_grant: bool,
    pub targtype: GrantTargetType,
    pub objtype: ObjectType,
    pub objects: *mut List,
    pub privileges: *mut List,
    pub grantees: *mut List,
    pub grant_option: bool,
    pub behavior: DropBehavior,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Group {
    pub plan: Plan,
    pub numCols: ::std::os::raw::c_int,
    pub grpColIdx: *mut AttrNumber,
    pub grpOperators: *mut Oid,
    pub grpCollations: *mut Oid,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GroupPathExtraData {
    pub flags: ::std::os::raw::c_int,
    pub partial_costs_set: bool,
    pub agg_partial_costs: AggClauseCosts,
    pub agg_final_costs: AggClauseCosts,
    pub target_parallel_safe: bool,
    pub havingQual: *mut Node,
    pub targetList: *mut List,
    pub patype: PartitionwiseAggregateType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GroupResultPath {
    pub path: Path,
    pub quals: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GroupState {
    pub ss: ScanState,
    pub eqfunction: *mut ExprState,
    pub grp_done: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hash {
    pub plan: Plan,
    pub hashkeys: *mut List,
    pub skewTable: Oid,
    pub skewColumn: AttrNumber,
    pub skewInherit: bool,
    pub rows_total: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashJoin {
    pub join: Join,
    pub hashclauses: *mut List,
    pub hashoperators: *mut List,
    pub hashcollations: *mut List,
    pub hashkeys: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashJoinState {
    pub js: JoinState,
    pub hashclauses: *mut ExprState,
    pub hj_OuterHashKeys: *mut List,
    pub hj_HashOperators: *mut List,
    pub hj_Collations: *mut List,
    pub hj_HashTable: HashJoinTable,
    pub hj_CurHashValue: uint32,
    pub hj_CurBucketNo: ::std::os::raw::c_int,
    pub hj_CurSkewBucketNo: ::std::os::raw::c_int,
    pub hj_CurTuple: HashJoinTuple,
    pub hj_OuterTupleSlot: *mut TupleTableSlot,
    pub hj_HashTupleSlot: *mut TupleTableSlot,
    pub hj_NullOuterTupleSlot: *mut TupleTableSlot,
    pub hj_NullInnerTupleSlot: *mut TupleTableSlot,
    pub hj_FirstOuterTupleSlot: *mut TupleTableSlot,
    pub hj_JoinState: ::std::os::raw::c_int,
    pub hj_MatchedOuter: bool,
    pub hj_OuterNotEmpty: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashPath {
    pub jpath: JoinPath,
    pub path_hashclauses: *mut List,
    pub num_batches: ::std::os::raw::c_int,
    pub inner_rows_total: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashState {
    pub ps: PlanState,
    pub hashtable: HashJoinTable,
    pub hashkeys: *mut List,
    pub shared_info: *mut SharedHashInfo,
    pub hinstrument: *mut HashInstrumentation,
    pub parallel_state: *mut ParallelHashJoinState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HeapTupleTableSlot {
    pub base: TupleTableSlot,
    pub tuple: HeapTuple,
    pub off: uint32,
    pub tupdata: HeapTupleData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexAmRoutine {
    pub type_: NodeTag,
    pub amstrategies: uint16,
    pub amsupport: uint16,
    pub amcanorder: bool,
    pub amcanorderbyop: bool,
    pub amcanbackward: bool,
    pub amcanunique: bool,
    pub amcanmulticol: bool,
    pub amoptionalkey: bool,
    pub amsearcharray: bool,
    pub amsearchnulls: bool,
    pub amstorage: bool,
    pub amclusterable: bool,
    pub ampredlocks: bool,
    pub amcanparallel: bool,
    pub amcaninclude: bool,
    pub amkeytype: Oid,
    pub ambuild: ambuild_function,
    pub ambuildempty: ambuildempty_function,
    pub aminsert: aminsert_function,
    pub ambulkdelete: ambulkdelete_function,
    pub amvacuumcleanup: amvacuumcleanup_function,
    pub amcanreturn: amcanreturn_function,
    pub amcostestimate: amcostestimate_function,
    pub amoptions: amoptions_function,
    pub amproperty: amproperty_function,
    pub ambuildphasename: ambuildphasename_function,
    pub amvalidate: amvalidate_function,
    pub ambeginscan: ambeginscan_function,
    pub amrescan: amrescan_function,
    pub amgettuple: amgettuple_function,
    pub amgetbitmap: amgetbitmap_function,
    pub amendscan: amendscan_function,
    pub ammarkpos: ammarkpos_function,
    pub amrestrpos: amrestrpos_function,
    pub amestimateparallelscan: amestimateparallelscan_function,
    pub aminitparallelscan: aminitparallelscan_function,
    pub amparallelrescan: amparallelrescan_function,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexArrayKeyInfo {
    pub scan_key: *mut ScanKeyData,
    pub array_expr: *mut ExprState,
    pub next_elem: ::std::os::raw::c_int,
    pub num_elems: ::std::os::raw::c_int,
    pub elem_values: *mut Datum,
    pub elem_nulls: *mut bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexClause {
    pub type_: NodeTag,
    pub rinfo: *mut RestrictInfo,
    pub indexquals: *mut List,
    pub lossy: bool,
    pub indexcol: AttrNumber,
    pub indexcols: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexFetchTableData {
    pub rel: Relation,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexInfo {
    pub type_: NodeTag,
    pub ii_NumIndexAttrs: ::std::os::raw::c_int,
    pub ii_NumIndexKeyAttrs: ::std::os::raw::c_int,
    pub ii_IndexAttrNumbers: [AttrNumber; 32usize],
    pub ii_Expressions: *mut List,
    pub ii_ExpressionsState: *mut List,
    pub ii_Predicate: *mut List,
    pub ii_PredicateState: *mut ExprState,
    pub ii_ExclusionOps: *mut Oid,
    pub ii_ExclusionProcs: *mut Oid,
    pub ii_ExclusionStrats: *mut uint16,
    pub ii_UniqueOps: *mut Oid,
    pub ii_UniqueProcs: *mut Oid,
    pub ii_UniqueStrats: *mut uint16,
    pub ii_Unique: bool,
    pub ii_ReadyForInserts: bool,
    pub ii_Concurrent: bool,
    pub ii_BrokenHotChain: bool,
    pub ii_ParallelWorkers: ::std::os::raw::c_int,
    pub ii_Am: Oid,
    pub ii_AmCache: *mut ::std::os::raw::c_void,
    pub ii_Context: MemoryContext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexOnlyScanState {
    pub ss: ScanState,
    pub indexqual: *mut ExprState,
    pub ioss_ScanKeys: *mut ScanKeyData,
    pub ioss_NumScanKeys: ::std::os::raw::c_int,
    pub ioss_OrderByKeys: *mut ScanKeyData,
    pub ioss_NumOrderByKeys: ::std::os::raw::c_int,
    pub ioss_RuntimeKeys: *mut IndexRuntimeKeyInfo,
    pub ioss_NumRuntimeKeys: ::std::os::raw::c_int,
    pub ioss_RuntimeKeysReady: bool,
    pub ioss_RuntimeContext: *mut ExprContext,
    pub ioss_RelationDesc: Relation,
    pub ioss_ScanDesc: *mut IndexScanDescData,
    pub ioss_TableSlot: *mut TupleTableSlot,
    pub ioss_VMBuffer: Buffer,
    pub ioss_PscanLen: Size,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexOptInfo {
    pub type_: NodeTag,
    pub indexoid: Oid,
    pub reltablespace: Oid,
    pub rel: *mut RelOptInfo,
    pub pages: BlockNumber,
    pub tuples: f64,
    pub tree_height: ::std::os::raw::c_int,
    pub ncolumns: ::std::os::raw::c_int,
    pub nkeycolumns: ::std::os::raw::c_int,
    pub indexkeys: *mut ::std::os::raw::c_int,
    pub indexcollations: *mut Oid,
    pub opfamily: *mut Oid,
    pub opcintype: *mut Oid,
    pub sortopfamily: *mut Oid,
    pub reverse_sort: *mut bool,
    pub nulls_first: *mut bool,
    pub canreturn: *mut bool,
    pub relam: Oid,
    pub indexprs: *mut List,
    pub indpred: *mut List,
    pub indextlist: *mut List,
    pub indrestrictinfo: *mut List,
    pub predOK: bool,
    pub unique: bool,
    pub immediate: bool,
    pub hypothetical: bool,
    pub amcanorderbyop: bool,
    pub amoptionalkey: bool,
    pub amsearcharray: bool,
    pub amsearchnulls: bool,
    pub amhasgettuple: bool,
    pub amhasgetbitmap: bool,
    pub amcanparallel: bool,
    pub amcostestimate: ::std::option::Option<unsafe extern "C" fn()>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexPath {
    pub path: Path,
    pub indexinfo: *mut IndexOptInfo,
    pub indexclauses: *mut List,
    pub indexorderbys: *mut List,
    pub indexorderbycols: *mut List,
    pub indexscandir: ScanDirection,
    pub indextotalcost: Cost,
    pub indexselectivity: Selectivity,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexRuntimeKeyInfo {
    pub scan_key: *mut ScanKeyData,
    pub key_expr: *mut ExprState,
    pub key_toastable: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexScanDescData {
    pub heapRelation: Relation,
    pub indexRelation: Relation,
    pub xs_snapshot: *mut SnapshotData,
    pub numberOfKeys: ::std::os::raw::c_int,
    pub numberOfOrderBys: ::std::os::raw::c_int,
    pub keyData: *mut ScanKeyData,
    pub orderByData: *mut ScanKeyData,
    pub xs_want_itup: bool,
    pub xs_temp_snap: bool,
    pub kill_prior_tuple: bool,
    pub ignore_killed_tuples: bool,
    pub xactStartedInRecovery: bool,
    pub opaque: *mut ::std::os::raw::c_void,
    pub xs_itup: IndexTuple,
    pub xs_itupdesc: *mut TupleDescData,
    pub xs_hitup: HeapTuple,
    pub xs_hitupdesc: *mut TupleDescData,
    pub xs_heaptid: ItemPointerData,
    pub xs_heap_continue: bool,
    pub xs_heapfetch: *mut IndexFetchTableData,
    pub xs_recheck: bool,
    pub xs_orderbyvals: *mut Datum,
    pub xs_orderbynulls: *mut bool,
    pub xs_recheckorderby: bool,
    pub parallel_scan: *mut ParallelIndexScanDescData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexScanState {
    pub ss: ScanState,
    pub indexqualorig: *mut ExprState,
    pub indexorderbyorig: *mut List,
    pub iss_ScanKeys: *mut ScanKeyData,
    pub iss_NumScanKeys: ::std::os::raw::c_int,
    pub iss_OrderByKeys: *mut ScanKeyData,
    pub iss_NumOrderByKeys: ::std::os::raw::c_int,
    pub iss_RuntimeKeys: *mut IndexRuntimeKeyInfo,
    pub iss_NumRuntimeKeys: ::std::os::raw::c_int,
    pub iss_RuntimeKeysReady: bool,
    pub iss_RuntimeContext: *mut ExprContext,
    pub iss_RelationDesc: Relation,
    pub iss_ScanDesc: *mut IndexScanDescData,
    pub iss_ReorderQueue: *mut pairingheap,
    pub iss_ReachedEnd: bool,
    pub iss_OrderByValues: *mut Datum,
    pub iss_OrderByNulls: *mut bool,
    pub iss_SortSupport: SortSupport,
    pub iss_OrderByTypByVals: *mut bool,
    pub iss_OrderByTypLens: *mut int16,
    pub iss_PscanLen: Size,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexStmt {
    pub type_: NodeTag,
    pub idxname: *mut ::std::os::raw::c_char,
    pub relation: *mut RangeVar,
    pub accessMethod: *mut ::std::os::raw::c_char,
    pub tableSpace: *mut ::std::os::raw::c_char,
    pub indexParams: *mut List,
    pub indexIncludingParams: *mut List,
    pub options: *mut List,
    pub whereClause: *mut Node,
    pub excludeOpNames: *mut List,
    pub idxcomment: *mut ::std::os::raw::c_char,
    pub indexOid: Oid,
    pub oldNode: Oid,
    pub unique: bool,
    pub primary: bool,
    pub isconstraint: bool,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub transformed: bool,
    pub concurrent: bool,
    pub if_not_exists: bool,
    pub reset_default_tblspc: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexVacuumInfo {
    pub index: Relation,
    pub analyze_only: bool,
    pub report_progress: bool,
    pub estimated_count: bool,
    pub message_level: ::std::os::raw::c_int,
    pub num_heap_tuples: f64,
    pub strategy: BufferAccessStrategy,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InlineCodeBlock {
    pub type_: NodeTag,
    pub source_text: *mut ::std::os::raw::c_char,
    pub langOid: Oid,
    pub langIsTrusted: bool,
    pub atomic: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InternalGrant {
    pub is_grant: bool,
    pub objtype: ObjectType,
    pub objects: *mut List,
    pub all_privs: bool,
    pub privileges: AclMode,
    pub col_privs: *mut List,
    pub grantees: *mut List,
    pub grant_option: bool,
    pub behavior: DropBehavior,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IntoClause {
    pub type_: NodeTag,
    pub rel: *mut RangeVar,
    pub colNames: *mut List,
    pub accessMethod: *mut ::std::os::raw::c_char,
    pub options: *mut List,
    pub onCommit: OnCommitAction,
    pub tableSpaceName: *mut ::std::os::raw::c_char,
    pub viewQuery: *mut Node,
    pub skipData: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LOCALLOCK {
    pub tag: LOCALLOCKTAG,
    pub hashcode: uint32,
    pub lock: *mut LOCK,
    pub proclock: *mut PROCLOCK,
    pub nLocks: int64,
    pub numLockOwners: ::std::os::raw::c_int,
    pub maxLockOwners: ::std::os::raw::c_int,
    pub lockOwners: *mut LOCALLOCKOWNER,
    pub holdsStrongLockCount: bool,
    pub lockCleared: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LockRowsState {
    pub ps: PlanState,
    pub lr_arowMarks: *mut List,
    pub lr_epqstate: EPQState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryContextData {
    pub type_: NodeTag,
    pub isReset: bool,
    pub allowInCritSection: bool,
    pub methods: *const MemoryContextMethods,
    pub parent: MemoryContext,
    pub firstchild: MemoryContext,
    pub prevchild: MemoryContext,
    pub nextchild: MemoryContext,
    pub name: *const ::std::os::raw::c_char,
    pub ident: *const ::std::os::raw::c_char,
    pub reset_cbs: *mut MemoryContextCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MergeAppend {
    pub plan: Plan,
    pub mergeplans: *mut List,
    pub numCols: ::std::os::raw::c_int,
    pub sortColIdx: *mut AttrNumber,
    pub sortOperators: *mut Oid,
    pub collations: *mut Oid,
    pub nullsFirst: *mut bool,
    pub part_prune_info: *mut PartitionPruneInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MergeAppendState {
    pub ps: PlanState,
    pub mergeplans: *mut *mut PlanState,
    pub ms_nplans: ::std::os::raw::c_int,
    pub ms_nkeys: ::std::os::raw::c_int,
    pub ms_sortkeys: SortSupport,
    pub ms_slots: *mut *mut TupleTableSlot,
    pub ms_heap: *mut binaryheap,
    pub ms_initialized: bool,
    pub ms_noopscan: bool,
    pub ms_prune_state: *mut PartitionPruneState,
    pub ms_valid_subplans: *mut Bitmapset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MinimalTupleTableSlot {
    pub base: TupleTableSlot,
    pub tuple: HeapTuple,
    pub mintuple: MinimalTuple,
    pub minhdr: HeapTupleData,
    pub off: uint32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModifyTable {
    pub plan: Plan,
    pub operation: CmdType,
    pub canSetTag: bool,
    pub nominalRelation: Index,
    pub rootRelation: Index,
    pub partColsUpdated: bool,
    pub resultRelations: *mut List,
    pub resultRelIndex: ::std::os::raw::c_int,
    pub rootResultRelIndex: ::std::os::raw::c_int,
    pub plans: *mut List,
    pub withCheckOptionLists: *mut List,
    pub returningLists: *mut List,
    pub fdwPrivLists: *mut List,
    pub fdwDirectModifyPlans: *mut Bitmapset,
    pub rowMarks: *mut List,
    pub epqParam: ::std::os::raw::c_int,
    pub onConflictAction: OnConflictAction,
    pub arbiterIndexes: *mut List,
    pub onConflictSet: *mut List,
    pub onConflictWhere: *mut Node,
    pub exclRelRTI: Index,
    pub exclRelTlist: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModifyTablePath {
    pub path: Path,
    pub operation: CmdType,
    pub canSetTag: bool,
    pub nominalRelation: Index,
    pub rootRelation: Index,
    pub partColsUpdated: bool,
    pub resultRelations: *mut List,
    pub subpaths: *mut List,
    pub subroots: *mut List,
    pub withCheckOptionLists: *mut List,
    pub returningLists: *mut List,
    pub rowMarks: *mut List,
    pub onconflict: *mut OnConflictExpr,
    pub epqParam: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModifyTableState {
    pub ps: PlanState,
    pub operation: CmdType,
    pub canSetTag: bool,
    pub mt_done: bool,
    pub mt_plans: *mut *mut PlanState,
    pub mt_nplans: ::std::os::raw::c_int,
    pub mt_whichplan: ::std::os::raw::c_int,
    pub mt_scans: *mut *mut TupleTableSlot,
    pub resultRelInfo: *mut ResultRelInfo,
    pub rootResultRelInfo: *mut ResultRelInfo,
    pub mt_arowmarks: *mut *mut List,
    pub mt_epqstate: EPQState,
    pub fireBSTriggers: bool,
    pub mt_excludedtlist: *mut List,
    pub mt_root_tuple_slot: *mut TupleTableSlot,
    pub mt_partition_tuple_routing: *mut PartitionTupleRouting,
    pub mt_transition_capture: *mut TransitionCaptureState,
    pub mt_oc_transition_capture: *mut TransitionCaptureState,
    pub mt_per_subplan_tupconv_maps: *mut *mut TupleConversionMap,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OnConflictSetState {
    pub type_: NodeTag,
    pub oc_Existing: *mut TupleTableSlot,
    pub oc_ProjSlot: *mut TupleTableSlot,
    pub oc_ProjInfo: *mut ProjectionInfo,
    pub oc_WhereClause: *mut ExprState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PROC_HDR {
    pub allProcs: *mut PGPROC,
    pub allPgXact: *mut PGXACT,
    pub allProcCount: uint32,
    pub freeProcs: *mut PGPROC,
    pub autovacFreeProcs: *mut PGPROC,
    pub bgworkerFreeProcs: *mut PGPROC,
    pub walsenderFreeProcs: *mut PGPROC,
    pub procArrayGroupFirst: pg_atomic_uint32,
    pub clogGroupFirst: pg_atomic_uint32,
    pub walwriterLatch: *mut Latch,
    pub checkpointerLatch: *mut Latch,
    pub spins_per_delay: ::std::os::raw::c_int,
    pub startupProc: *mut PGPROC,
    pub startupProcPid: ::std::os::raw::c_int,
    pub startupBufferPinWaitBufId: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParallelAppendState {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParallelContext {
    pub node: dlist_node,
    pub subid: SubTransactionId,
    pub nworkers: ::std::os::raw::c_int,
    pub nworkers_launched: ::std::os::raw::c_int,
    pub library_name: *mut ::std::os::raw::c_char,
    pub function_name: *mut ::std::os::raw::c_char,
    pub error_context_stack: *mut ErrorContextCallback,
    pub estimator: shm_toc_estimator,
    pub seg: *mut dsm_segment,
    pub private_memory: *mut ::std::os::raw::c_void,
    pub toc: *mut shm_toc,
    pub worker: *mut ParallelWorkerInfo,
    pub nknown_attached_workers: ::std::os::raw::c_int,
    pub known_attached_workers: *mut bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParallelHashJoinState {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParallelWorkerContext {
    pub seg: *mut dsm_segment,
    pub toc: *mut shm_toc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionBoundSpec {
    pub type_: NodeTag,
    pub strategy: ::std::os::raw::c_char,
    pub is_default: bool,
    pub modulus: ::std::os::raw::c_int,
    pub remainder: ::std::os::raw::c_int,
    pub listdatums: *mut List,
    pub lowerdatums: *mut List,
    pub upperdatums: *mut List,
    pub location: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionDescData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionDirectoryData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionKeyData {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionPruneInfo {
    pub type_: NodeTag,
    pub prune_infos: *mut List,
    pub other_subplans: *mut Bitmapset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionPruneState {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionPruneStep {
    pub type_: NodeTag,
    pub step_id: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionPruneStepCombine {
    pub step: PartitionPruneStep,
    pub combineOp: PartitionPruneCombineOp,
    pub source_stepids: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionPruneStepOp {
    pub step: PartitionPruneStep,
    pub opstrategy: StrategyNumber,
    pub exprs: *mut List,
    pub cmpfns: *mut List,
    pub nullkeys: *mut Bitmapset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionRoutingInfo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionSchemeData {
    pub strategy: ::std::os::raw::c_char,
    pub partnatts: int16,
    pub partopfamily: *mut Oid,
    pub partopcintype: *mut Oid,
    pub partcollation: *mut Oid,
    pub parttyplen: *mut int16,
    pub parttypbyval: *mut bool,
    pub partsupfunc: *mut FmgrInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionedRelPruneInfo {
    pub type_: NodeTag,
    pub rtindex: Index,
    pub present_parts: *mut Bitmapset,
    pub nparts: ::std::os::raw::c_int,
    pub subplan_map: *mut ::std::os::raw::c_int,
    pub subpart_map: *mut ::std::os::raw::c_int,
    pub relid_map: *mut Oid,
    pub initial_pruning_steps: *mut List,
    pub exec_pruning_steps: *mut List,
    pub execparamids: *mut Bitmapset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PgStat_MsgChecksumFailure {
    pub m_hdr: PgStat_MsgHdr,
    pub m_databaseid: Oid,
    pub m_failurecount: ::std::os::raw::c_int,
    pub m_failure_time: TimestampTz,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PgStat_StatDBEntry {
    pub databaseid: Oid,
    pub n_xact_commit: PgStat_Counter,
    pub n_xact_rollback: PgStat_Counter,
    pub n_blocks_fetched: PgStat_Counter,
    pub n_blocks_hit: PgStat_Counter,
    pub n_tuples_returned: PgStat_Counter,
    pub n_tuples_fetched: PgStat_Counter,
    pub n_tuples_inserted: PgStat_Counter,
    pub n_tuples_updated: PgStat_Counter,
    pub n_tuples_deleted: PgStat_Counter,
    pub last_autovac_time: TimestampTz,
    pub n_conflict_tablespace: PgStat_Counter,
    pub n_conflict_lock: PgStat_Counter,
    pub n_conflict_snapshot: PgStat_Counter,
    pub n_conflict_bufferpin: PgStat_Counter,
    pub n_conflict_startup_deadlock: PgStat_Counter,
    pub n_temp_files: PgStat_Counter,
    pub n_temp_bytes: PgStat_Counter,
    pub n_deadlocks: PgStat_Counter,
    pub n_checksum_failures: PgStat_Counter,
    pub last_checksum_failure: TimestampTz,
    pub n_block_read_time: PgStat_Counter,
    pub n_block_write_time: PgStat_Counter,
    pub stat_reset_timestamp: TimestampTz,
    pub stats_timestamp: TimestampTz,
    pub tables: *mut HTAB,
    pub functions: *mut HTAB,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlanState {
    pub type_: NodeTag,
    pub plan: *mut Plan,
    pub state: *mut EState,
    pub ExecProcNode: ExecProcNodeMtd,
    pub ExecProcNodeReal: ExecProcNodeMtd,
    pub instrument: *mut Instrumentation,
    pub worker_instrument: *mut WorkerInstrumentation,
    pub worker_jit_instrument: *mut SharedJitInstrumentation,
    pub qual: *mut ExprState,
    pub lefttree: *mut PlanState,
    pub righttree: *mut PlanState,
    pub initPlan: *mut List,
    pub subPlan: *mut List,
    pub chgParam: *mut Bitmapset,
    pub ps_ResultTupleDesc: TupleDesc,
    pub ps_ResultTupleSlot: *mut TupleTableSlot,
    pub ps_ExprContext: *mut ExprContext,
    pub ps_ProjInfo: *mut ProjectionInfo,
    pub scandesc: TupleDesc,
    pub scanops: *const TupleTableSlotOps,
    pub outerops: *const TupleTableSlotOps,
    pub innerops: *const TupleTableSlotOps,
    pub resultops: *const TupleTableSlotOps,
    pub scanopsfixed: bool,
    pub outeropsfixed: bool,
    pub inneropsfixed: bool,
    pub resultopsfixed: bool,
    pub scanopsset: bool,
    pub outeropsset: bool,
    pub inneropsset: bool,
    pub resultopsset: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlannedStmt {
    pub type_: NodeTag,
    pub commandType: CmdType,
    pub queryId: uint64,
    pub hasReturning: bool,
    pub hasModifyingCTE: bool,
    pub canSetTag: bool,
    pub transientPlan: bool,
    pub dependsOnRole: bool,
    pub parallelModeNeeded: bool,
    pub jitFlags: ::std::os::raw::c_int,
    pub planTree: *mut Plan,
    pub rtable: *mut List,
    pub resultRelations: *mut List,
    pub rootResultRelations: *mut List,
    pub subplans: *mut List,
    pub rewindPlanIDs: *mut Bitmapset,
    pub rowMarks: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub paramExecTypes: *mut List,
    pub utilityStmt: *mut Node,
    pub stmt_location: ::std::os::raw::c_int,
    pub stmt_len: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlannerGlobal {
    pub type_: NodeTag,
    pub boundParams: ParamListInfo,
    pub subplans: *mut List,
    pub subroots: *mut List,
    pub rewindPlanIDs: *mut Bitmapset,
    pub finalrtable: *mut List,
    pub finalrowmarks: *mut List,
    pub resultRelations: *mut List,
    pub rootResultRelations: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub paramExecTypes: *mut List,
    pub lastPHId: Index,
    pub lastRowMarkId: Index,
    pub lastPlanNodeId: ::std::os::raw::c_int,
    pub transientPlan: bool,
    pub dependsOnRole: bool,
    pub parallelModeOK: bool,
    pub parallelModeNeeded: bool,
    pub maxParallelHazard: ::std::os::raw::c_char,
    pub partition_directory: PartitionDirectory,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlannerInfo {
    pub type_: NodeTag,
    pub parse: *mut Query,
    pub glob: *mut PlannerGlobal,
    pub query_level: Index,
    pub parent_root: *mut PlannerInfo,
    pub plan_params: *mut List,
    pub outer_params: *mut Bitmapset,
    pub simple_rel_array: *mut *mut RelOptInfo,
    pub simple_rel_array_size: ::std::os::raw::c_int,
    pub simple_rte_array: *mut *mut RangeTblEntry,
    pub append_rel_array: *mut *mut AppendRelInfo,
    pub all_baserels: Relids,
    pub nullable_baserels: Relids,
    pub join_rel_list: *mut List,
    pub join_rel_hash: *mut HTAB,
    pub join_rel_level: *mut *mut List,
    pub join_cur_level: ::std::os::raw::c_int,
    pub init_plans: *mut List,
    pub cte_plan_ids: *mut List,
    pub multiexpr_params: *mut List,
    pub eq_classes: *mut List,
    pub canon_pathkeys: *mut List,
    pub left_join_clauses: *mut List,
    pub right_join_clauses: *mut List,
    pub full_join_clauses: *mut List,
    pub join_info_list: *mut List,
    pub append_rel_list: *mut List,
    pub rowMarks: *mut List,
    pub placeholder_list: *mut List,
    pub fkey_list: *mut List,
    pub query_pathkeys: *mut List,
    pub group_pathkeys: *mut List,
    pub window_pathkeys: *mut List,
    pub distinct_pathkeys: *mut List,
    pub sort_pathkeys: *mut List,
    pub part_schemes: *mut List,
    pub initial_rels: *mut List,
    pub upper_rels: [*mut List; 7usize],
    pub upper_targets: [*mut PathTarget; 7usize],
    pub processed_tlist: *mut List,
    pub grouping_map: *mut AttrNumber,
    pub minmax_aggs: *mut List,
    pub planner_cxt: MemoryContext,
    pub total_table_pages: f64,
    pub tuple_fraction: f64,
    pub limit_tuples: f64,
    pub qual_security_level: Index,
    pub inhTargetKind: InheritanceKind,
    pub hasJoinRTEs: bool,
    pub hasLateralRTEs: bool,
    pub hasHavingQual: bool,
    pub hasPseudoConstantQuals: bool,
    pub hasRecursion: bool,
    pub wt_param_id: ::std::os::raw::c_int,
    pub non_recursive_path: *mut Path,
    pub curOuterRels: Relids,
    pub curOuterParams: *mut List,
    pub join_search_private: *mut ::std::os::raw::c_void,
    pub partColsUpdated: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PortalData {
    pub name: *const ::std::os::raw::c_char,
    pub prepStmtName: *const ::std::os::raw::c_char,
    pub portalContext: MemoryContext,
    pub resowner: ResourceOwner,
    pub cleanup: ::std::option::Option<unsafe extern "C" fn(portal: Portal)>,
    pub createSubid: SubTransactionId,
    pub activeSubid: SubTransactionId,
    pub sourceText: *const ::std::os::raw::c_char,
    pub commandTag: *const ::std::os::raw::c_char,
    pub stmts: *mut List,
    pub cplan: *mut CachedPlan,
    pub portalParams: ParamListInfo,
    pub queryEnv: *mut QueryEnvironment,
    pub strategy: PortalStrategy,
    pub cursorOptions: ::std::os::raw::c_int,
    pub run_once: bool,
    pub status: PortalStatus,
    pub portalPinned: bool,
    pub autoHeld: bool,
    pub queryDesc: *mut QueryDesc,
    pub tupDesc: TupleDesc,
    pub formats: *mut int16,
    pub holdStore: *mut Tuplestorestate,
    pub holdContext: MemoryContext,
    pub holdSnapshot: Snapshot,
    pub atStart: bool,
    pub atEnd: bool,
    pub portalPos: uint64,
    pub creation_time: TimestampTz,
    pub visible: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProjectSetState {
    pub ps: PlanState,
    pub elems: *mut *mut Node,
    pub elemdone: *mut ExprDoneCond,
    pub nelems: ::std::os::raw::c_int,
    pub pending_srf_tuples: bool,
    pub argcontext: MemoryContext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RangeTblEntry {
    pub type_: NodeTag,
    pub rtekind: RTEKind,
    pub relid: Oid,
    pub relkind: ::std::os::raw::c_char,
    pub rellockmode: ::std::os::raw::c_int,
    pub tablesample: *mut TableSampleClause,
    pub subquery: *mut Query,
    pub security_barrier: bool,
    pub jointype: JoinType,
    pub joinaliasvars: *mut List,
    pub functions: *mut List,
    pub funcordinality: bool,
    pub tablefunc: *mut TableFunc,
    pub values_lists: *mut List,
    pub ctename: *mut ::std::os::raw::c_char,
    pub ctelevelsup: Index,
    pub self_reference: bool,
    pub coltypes: *mut List,
    pub coltypmods: *mut List,
    pub colcollations: *mut List,
    pub enrname: *mut ::std::os::raw::c_char,
    pub enrtuples: f64,
    pub alias: *mut Alias,
    pub eref: *mut Alias,
    pub lateral: bool,
    pub inh: bool,
    pub inFromCl: bool,
    pub requiredPerms: AclMode,
    pub checkAsUser: Oid,
    pub selectedCols: *mut Bitmapset,
    pub insertedCols: *mut Bitmapset,
    pub updatedCols: *mut Bitmapset,
    pub extraUpdatedCols: *mut Bitmapset,
    pub securityQuals: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RecursiveUnion {
    pub plan: Plan,
    pub wtParam: ::std::os::raw::c_int,
    pub numCols: ::std::os::raw::c_int,
    pub dupColIdx: *mut AttrNumber,
    pub dupOperators: *mut Oid,
    pub dupCollations: *mut Oid,
    pub numGroups: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RecursiveUnionState {
    pub ps: PlanState,
    pub recursing: bool,
    pub intermediate_empty: bool,
    pub working_table: *mut Tuplestorestate,
    pub intermediate_table: *mut Tuplestorestate,
    pub eqfuncoids: *mut Oid,
    pub hashfunctions: *mut FmgrInfo,
    pub tempContext: MemoryContext,
    pub hashtable: TupleHashTable,
    pub tableContext: MemoryContext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ReindexStmt {
    pub type_: NodeTag,
    pub kind: ReindexObjectType,
    pub relation: *mut RangeVar,
    pub name: *const ::std::os::raw::c_char,
    pub options: ::std::os::raw::c_int,
    pub concurrent: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RelOptInfo {
    pub type_: NodeTag,
    pub reloptkind: RelOptKind,
    pub relids: Relids,
    pub rows: f64,
    pub consider_startup: bool,
    pub consider_param_startup: bool,
    pub consider_parallel: bool,
    pub reltarget: *mut PathTarget,
    pub pathlist: *mut List,
    pub ppilist: *mut List,
    pub partial_pathlist: *mut List,
    pub cheapest_startup_path: *mut Path,
    pub cheapest_total_path: *mut Path,
    pub cheapest_unique_path: *mut Path,
    pub cheapest_parameterized_paths: *mut List,
    pub direct_lateral_relids: Relids,
    pub lateral_relids: Relids,
    pub relid: Index,
    pub reltablespace: Oid,
    pub rtekind: RTEKind,
    pub min_attr: AttrNumber,
    pub max_attr: AttrNumber,
    pub attr_needed: *mut Relids,
    pub attr_widths: *mut int32,
    pub lateral_vars: *mut List,
    pub lateral_referencers: Relids,
    pub indexlist: *mut List,
    pub statlist: *mut List,
    pub pages: BlockNumber,
    pub tuples: f64,
    pub allvisfrac: f64,
    pub subroot: *mut PlannerInfo,
    pub subplan_params: *mut List,
    pub rel_parallel_workers: ::std::os::raw::c_int,
    pub serverid: Oid,
    pub userid: Oid,
    pub useridiscurrent: bool,
    pub fdwroutine: *mut FdwRoutine,
    pub fdw_private: *mut ::std::os::raw::c_void,
    pub unique_for_rels: *mut List,
    pub non_unique_for_rels: *mut List,
    pub baserestrictinfo: *mut List,
    pub baserestrictcost: QualCost,
    pub baserestrict_min_security: Index,
    pub joininfo: *mut List,
    pub has_eclass_joins: bool,
    pub consider_partitionwise_join: bool,
    pub top_parent_relids: Relids,
    pub part_scheme: PartitionScheme,
    pub nparts: ::std::os::raw::c_int,
    pub boundinfo: *mut PartitionBoundInfoData,
    pub partition_qual: *mut List,
    pub part_rels: *mut *mut RelOptInfo,
    pub partexprs: *mut *mut List,
    pub nullable_partexprs: *mut *mut List,
    pub partitioned_child_rels: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RelationData {
    pub rd_node: RelFileNode,
    pub rd_smgr: *mut SMgrRelationData,
    pub rd_refcnt: ::std::os::raw::c_int,
    pub rd_backend: BackendId,
    pub rd_islocaltemp: bool,
    pub rd_isnailed: bool,
    pub rd_isvalid: bool,
    pub rd_indexvalid: bool,
    pub rd_statvalid: bool,
    pub rd_createSubid: SubTransactionId,
    pub rd_newRelfilenodeSubid: SubTransactionId,
    pub rd_rel: Form_pg_class,
    pub rd_att: TupleDesc,
    pub rd_id: Oid,
    pub rd_lockInfo: LockInfoData,
    pub rd_rules: *mut RuleLock,
    pub rd_rulescxt: MemoryContext,
    pub trigdesc: *mut TriggerDesc,
    pub rd_rsdesc: *mut RowSecurityDesc,
    pub rd_fkeylist: *mut List,
    pub rd_fkeyvalid: bool,
    pub rd_partkey: *mut PartitionKeyData,
    pub rd_partkeycxt: MemoryContext,
    pub rd_partdesc: *mut PartitionDescData,
    pub rd_pdcxt: MemoryContext,
    pub rd_partcheck: *mut List,
    pub rd_partcheckvalid: bool,
    pub rd_partcheckcxt: MemoryContext,
    pub rd_indexlist: *mut List,
    pub rd_pkindex: Oid,
    pub rd_replidindex: Oid,
    pub rd_statlist: *mut List,
    pub rd_indexattr: *mut Bitmapset,
    pub rd_keyattr: *mut Bitmapset,
    pub rd_pkattr: *mut Bitmapset,
    pub rd_idattr: *mut Bitmapset,
    pub rd_pubactions: *mut PublicationActions,
    pub rd_options: *mut bytea,
    pub rd_amhandler: Oid,
    pub rd_tableam: *const TableAmRoutine,
    pub rd_index: Form_pg_index,
    pub rd_indextuple: *mut HeapTupleData,
    pub rd_indexcxt: MemoryContext,
    pub rd_indam: *mut IndexAmRoutine,
    pub rd_opfamily: *mut Oid,
    pub rd_opcintype: *mut Oid,
    pub rd_support: *mut RegProcedure,
    pub rd_supportinfo: *mut FmgrInfo,
    pub rd_indoption: *mut int16,
    pub rd_indexprs: *mut List,
    pub rd_indpred: *mut List,
    pub rd_exclops: *mut Oid,
    pub rd_exclprocs: *mut Oid,
    pub rd_exclstrats: *mut uint16,
    pub rd_indcollation: *mut Oid,
    pub rd_amcache: *mut ::std::os::raw::c_void,
    pub rd_fdwroutine: *mut FdwRoutine,
    pub rd_toastoid: Oid,
    pub pgstat_info: *mut PgStat_TableStatus,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RestrictInfo {
    pub type_: NodeTag,
    pub clause: *mut Expr,
    pub is_pushed_down: bool,
    pub outerjoin_delayed: bool,
    pub can_join: bool,
    pub pseudoconstant: bool,
    pub leakproof: bool,
    pub security_level: Index,
    pub clause_relids: Relids,
    pub required_relids: Relids,
    pub outer_relids: Relids,
    pub nullable_relids: Relids,
    pub left_relids: Relids,
    pub right_relids: Relids,
    pub orclause: *mut Expr,
    pub parent_ec: *mut EquivalenceClass,
    pub eval_cost: QualCost,
    pub norm_selec: Selectivity,
    pub outer_selec: Selectivity,
    pub mergeopfamilies: *mut List,
    pub left_ec: *mut EquivalenceClass,
    pub right_ec: *mut EquivalenceClass,
    pub left_em: *mut EquivalenceMember,
    pub right_em: *mut EquivalenceMember,
    pub scansel_cache: *mut List,
    pub outer_is_left: bool,
    pub hashjoinoperator: Oid,
    pub left_bucketsize: Selectivity,
    pub right_bucketsize: Selectivity,
    pub left_mcvfreq: Selectivity,
    pub right_mcvfreq: Selectivity,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResultRelInfo {
    pub type_: NodeTag,
    pub ri_RangeTableIndex: Index,
    pub ri_RelationDesc: Relation,
    pub ri_NumIndices: ::std::os::raw::c_int,
    pub ri_IndexRelationDescs: RelationPtr,
    pub ri_IndexRelationInfo: *mut *mut IndexInfo,
    pub ri_TrigDesc: *mut TriggerDesc,
    pub ri_TrigFunctions: *mut FmgrInfo,
    pub ri_TrigWhenExprs: *mut *mut ExprState,
    pub ri_TrigInstrument: *mut Instrumentation,
    pub ri_ReturningSlot: *mut TupleTableSlot,
    pub ri_TrigOldSlot: *mut TupleTableSlot,
    pub ri_TrigNewSlot: *mut TupleTableSlot,
    pub ri_FdwRoutine: *mut FdwRoutine,
    pub ri_FdwState: *mut ::std::os::raw::c_void,
    pub ri_usesFdwDirectModify: bool,
    pub ri_WithCheckOptions: *mut List,
    pub ri_WithCheckOptionExprs: *mut List,
    pub ri_ConstraintExprs: *mut *mut ExprState,
    pub ri_GeneratedExprs: *mut *mut ExprState,
    pub ri_junkFilter: *mut JunkFilter,
    pub ri_returningList: *mut List,
    pub ri_projectReturning: *mut ProjectionInfo,
    pub ri_onConflictArbiterIndexes: *mut List,
    pub ri_onConflict: *mut OnConflictSetState,
    pub ri_PartitionCheck: *mut List,
    pub ri_PartitionCheckExpr: *mut ExprState,
    pub ri_PartitionRoot: Relation,
    pub ri_PartitionInfo: *mut PartitionRoutingInfo,
    pub ri_CopyMultiInsertBuffer: *mut CopyMultiInsertBuffer,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SampleScanState {
    pub ss: ScanState,
    pub args: *mut List,
    pub repeatable: *mut ExprState,
    pub tsmroutine: *mut TsmRoutine,
    pub tsm_state: *mut ::std::os::raw::c_void,
    pub use_bulkread: bool,
    pub use_pagemode: bool,
    pub begun: bool,
    pub seed: uint32,
    pub donetuples: int64,
    pub haveblock: bool,
    pub done: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScanState {
    pub ps: PlanState,
    pub ss_currentRelation: Relation,
    pub ss_currentScanDesc: *mut TableScanDescData,
    pub ss_ScanTupleSlot: *mut TupleTableSlot,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SetExprState {
    pub type_: NodeTag,
    pub expr: *mut Expr,
    pub args: *mut List,
    pub elidedFuncState: *mut ExprState,
    pub func: FmgrInfo,
    pub funcResultStore: *mut Tuplestorestate,
    pub funcResultSlot: *mut TupleTableSlot,
    pub funcResultDesc: TupleDesc,
    pub funcReturnsTuple: bool,
    pub funcReturnsSet: bool,
    pub setArgsValid: bool,
    pub shutdown_reg: bool,
    pub fcinfo: FunctionCallInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SetOp {
    pub plan: Plan,
    pub cmd: SetOpCmd,
    pub strategy: SetOpStrategy,
    pub numCols: ::std::os::raw::c_int,
    pub dupColIdx: *mut AttrNumber,
    pub dupOperators: *mut Oid,
    pub dupCollations: *mut Oid,
    pub flagColIdx: AttrNumber,
    pub firstFlag: ::std::os::raw::c_int,
    pub numGroups: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SetOpState {
    pub ps: PlanState,
    pub eqfunction: *mut ExprState,
    pub eqfuncoids: *mut Oid,
    pub hashfunctions: *mut FmgrInfo,
    pub setop_done: bool,
    pub numOutput: ::std::os::raw::c_long,
    pub pergroup: SetOpStatePerGroup,
    pub grp_firstTuple: HeapTuple,
    pub hashtable: TupleHashTable,
    pub tableContext: MemoryContext,
    pub table_filled: bool,
    pub hashiter: TupleHashIterator,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SharedRecordTypmodRegistry {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SharedTuplestore {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SharedTuplestoreAccessor {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sharedsort {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SnapshotData {
    pub snapshot_type: SnapshotType,
    pub xmin: TransactionId,
    pub xmax: TransactionId,
    pub xip: *mut TransactionId,
    pub xcnt: uint32,
    pub subxip: *mut TransactionId,
    pub subxcnt: int32,
    pub suboverflowed: bool,
    pub takenDuringRecovery: bool,
    pub copied: bool,
    pub curcid: CommandId,
    pub speculativeToken: uint32,
    pub active_count: uint32,
    pub regd_count: uint32,
    pub ph_node: pairingheap_node,
    pub whenTaken: TimestampTz,
    pub lsn: XLogRecPtr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SortCoordinateData {
    pub isWorker: bool,
    pub nParticipants: ::std::os::raw::c_int,
    pub sharedsort: *mut Sharedsort,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SortState {
    pub ss: ScanState,
    pub randomAccess: bool,
    pub bounded: bool,
    pub bound: int64,
    pub sort_Done: bool,
    pub bounded_Done: bool,
    pub bound_Done: int64,
    pub tuplesortstate: *mut ::std::os::raw::c_void,
    pub am_worker: bool,
    pub shared_info: *mut SharedSortInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SubPlanState {
    pub type_: NodeTag,
    pub subplan: *mut SubPlan,
    pub planstate: *mut PlanState,
    pub parent: *mut PlanState,
    pub testexpr: *mut ExprState,
    pub args: *mut List,
    pub curTuple: HeapTuple,
    pub curArray: Datum,
    pub descRight: TupleDesc,
    pub projLeft: *mut ProjectionInfo,
    pub projRight: *mut ProjectionInfo,
    pub hashtable: TupleHashTable,
    pub hashnulls: TupleHashTable,
    pub havehashrows: bool,
    pub havenullrows: bool,
    pub hashtablecxt: MemoryContext,
    pub hashtempcxt: MemoryContext,
    pub innerecontext: *mut ExprContext,
    pub keyColIdx: *mut AttrNumber,
    pub tab_eq_funcoids: *mut Oid,
    pub tab_collations: *mut Oid,
    pub tab_hash_funcs: *mut FmgrInfo,
    pub tab_eq_funcs: *mut FmgrInfo,
    pub lhs_hash_funcs: *mut FmgrInfo,
    pub cur_eq_funcs: *mut FmgrInfo,
    pub cur_eq_comp: *mut ExprState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SubscriptingRef {
    pub xpr: Expr,
    pub refcontainertype: Oid,
    pub refelemtype: Oid,
    pub reftypmod: int32,
    pub refcollid: Oid,
    pub refupperindexpr: *mut List,
    pub reflowerindexpr: *mut List,
    pub refexpr: *mut Expr,
    pub refassgnexpr: *mut Expr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SupportRequestCost {
    pub type_: NodeTag,
    pub root: *mut PlannerInfo,
    pub funcid: Oid,
    pub node: *mut Node,
    pub startup: Cost,
    pub per_tuple: Cost,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SupportRequestIndexCondition {
    pub type_: NodeTag,
    pub root: *mut PlannerInfo,
    pub funcid: Oid,
    pub node: *mut Node,
    pub indexarg: ::std::os::raw::c_int,
    pub index: *mut IndexOptInfo,
    pub indexcol: ::std::os::raw::c_int,
    pub opfamily: Oid,
    pub indexcollation: Oid,
    pub lossy: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SupportRequestRows {
    pub type_: NodeTag,
    pub root: *mut PlannerInfo,
    pub funcid: Oid,
    pub node: *mut Node,
    pub rows: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SupportRequestSelectivity {
    pub type_: NodeTag,
    pub root: *mut PlannerInfo,
    pub funcid: Oid,
    pub args: *mut List,
    pub inputcollid: Oid,
    pub is_join: bool,
    pub varRelid: ::std::os::raw::c_int,
    pub jointype: JoinType,
    pub sjinfo: *mut SpecialJoinInfo,
    pub selectivity: Selectivity,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SupportRequestSimplify {
    pub type_: NodeTag,
    pub root: *mut PlannerInfo,
    pub fcall: *mut FuncExpr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SysScanDescData {
    pub heap_rel: Relation,
    pub irel: Relation,
    pub scan: *mut TableScanDescData,
    pub iscan: *mut IndexScanDescData,
    pub snapshot: *mut SnapshotData,
    pub slot: *mut TupleTableSlot,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TableAmRoutine {
    pub type_: NodeTag,
    pub slot_callbacks:
        ::std::option::Option<unsafe extern "C" fn(rel: Relation) -> *const TupleTableSlotOps>,
    pub scan_begin: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            snapshot: Snapshot,
            nkeys: ::std::os::raw::c_int,
            key: *mut ScanKeyData,
            pscan: ParallelTableScanDesc,
            flags: uint32,
        ) -> TableScanDesc,
    >,
    pub scan_end: ::std::option::Option<unsafe extern "C" fn(scan: TableScanDesc)>,
    pub scan_rescan: ::std::option::Option<
        unsafe extern "C" fn(
            scan: TableScanDesc,
            key: *mut ScanKeyData,
            set_params: bool,
            allow_strat: bool,
            allow_sync: bool,
            allow_pagemode: bool,
        ),
    >,
    pub scan_getnextslot: ::std::option::Option<
        unsafe extern "C" fn(
            scan: TableScanDesc,
            direction: ScanDirection,
            slot: *mut TupleTableSlot,
        ) -> bool,
    >,
    pub parallelscan_estimate: ::std::option::Option<unsafe extern "C" fn(rel: Relation) -> Size>,
    pub parallelscan_initialize: ::std::option::Option<
        unsafe extern "C" fn(rel: Relation, pscan: ParallelTableScanDesc) -> Size,
    >,
    pub parallelscan_reinitialize:
        ::std::option::Option<unsafe extern "C" fn(rel: Relation, pscan: ParallelTableScanDesc)>,
    pub index_fetch_begin:
        ::std::option::Option<unsafe extern "C" fn(rel: Relation) -> *mut IndexFetchTableData>,
    pub index_fetch_reset:
        ::std::option::Option<unsafe extern "C" fn(data: *mut IndexFetchTableData)>,
    pub index_fetch_end:
        ::std::option::Option<unsafe extern "C" fn(data: *mut IndexFetchTableData)>,
    pub index_fetch_tuple: ::std::option::Option<
        unsafe extern "C" fn(
            scan: *mut IndexFetchTableData,
            tid: ItemPointer,
            snapshot: Snapshot,
            slot: *mut TupleTableSlot,
            call_again: *mut bool,
            all_dead: *mut bool,
        ) -> bool,
    >,
    pub tuple_fetch_row_version: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            tid: ItemPointer,
            snapshot: Snapshot,
            slot: *mut TupleTableSlot,
        ) -> bool,
    >,
    pub tuple_tid_valid:
        ::std::option::Option<unsafe extern "C" fn(scan: TableScanDesc, tid: ItemPointer) -> bool>,
    pub tuple_get_latest_tid:
        ::std::option::Option<unsafe extern "C" fn(scan: TableScanDesc, tid: ItemPointer)>,
    pub tuple_satisfies_snapshot: ::std::option::Option<
        unsafe extern "C" fn(rel: Relation, slot: *mut TupleTableSlot, snapshot: Snapshot) -> bool,
    >,
    pub compute_xid_horizon_for_tuples: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            items: *mut ItemPointerData,
            nitems: ::std::os::raw::c_int,
        ) -> TransactionId,
    >,
    pub tuple_insert: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            slot: *mut TupleTableSlot,
            cid: CommandId,
            options: ::std::os::raw::c_int,
            bistate: *mut BulkInsertStateData,
        ),
    >,
    pub tuple_insert_speculative: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            slot: *mut TupleTableSlot,
            cid: CommandId,
            options: ::std::os::raw::c_int,
            bistate: *mut BulkInsertStateData,
            specToken: uint32,
        ),
    >,
    pub tuple_complete_speculative: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            slot: *mut TupleTableSlot,
            specToken: uint32,
            succeeded: bool,
        ),
    >,
    pub multi_insert: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            slots: *mut *mut TupleTableSlot,
            nslots: ::std::os::raw::c_int,
            cid: CommandId,
            options: ::std::os::raw::c_int,
            bistate: *mut BulkInsertStateData,
        ),
    >,
    pub tuple_delete: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            tid: ItemPointer,
            cid: CommandId,
            snapshot: Snapshot,
            crosscheck: Snapshot,
            wait: bool,
            tmfd: *mut TM_FailureData,
            changingPart: bool,
        ) -> TM_Result,
    >,
    pub tuple_update: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            otid: ItemPointer,
            slot: *mut TupleTableSlot,
            cid: CommandId,
            snapshot: Snapshot,
            crosscheck: Snapshot,
            wait: bool,
            tmfd: *mut TM_FailureData,
            lockmode: *mut LockTupleMode,
            update_indexes: *mut bool,
        ) -> TM_Result,
    >,
    pub tuple_lock: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            tid: ItemPointer,
            snapshot: Snapshot,
            slot: *mut TupleTableSlot,
            cid: CommandId,
            mode: LockTupleMode,
            wait_policy: LockWaitPolicy,
            flags: uint8,
            tmfd: *mut TM_FailureData,
        ) -> TM_Result,
    >,
    pub finish_bulk_insert:
        ::std::option::Option<unsafe extern "C" fn(rel: Relation, options: ::std::os::raw::c_int)>,
    pub relation_set_new_filenode: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            newrnode: *const RelFileNode,
            persistence: ::std::os::raw::c_char,
            freezeXid: *mut TransactionId,
            minmulti: *mut MultiXactId,
        ),
    >,
    pub relation_nontransactional_truncate:
        ::std::option::Option<unsafe extern "C" fn(rel: Relation)>,
    pub relation_copy_data:
        ::std::option::Option<unsafe extern "C" fn(rel: Relation, newrnode: *const RelFileNode)>,
    pub relation_copy_for_cluster: ::std::option::Option<
        unsafe extern "C" fn(
            NewTable: Relation,
            OldTable: Relation,
            OldIndex: Relation,
            use_sort: bool,
            OldestXmin: TransactionId,
            xid_cutoff: *mut TransactionId,
            multi_cutoff: *mut MultiXactId,
            num_tuples: *mut f64,
            tups_vacuumed: *mut f64,
            tups_recently_dead: *mut f64,
        ),
    >,
    pub relation_vacuum: ::std::option::Option<
        unsafe extern "C" fn(
            onerel: Relation,
            params: *mut VacuumParams,
            bstrategy: BufferAccessStrategy,
        ),
    >,
    pub scan_analyze_next_block: ::std::option::Option<
        unsafe extern "C" fn(
            scan: TableScanDesc,
            blockno: BlockNumber,
            bstrategy: BufferAccessStrategy,
        ) -> bool,
    >,
    pub scan_analyze_next_tuple: ::std::option::Option<
        unsafe extern "C" fn(
            scan: TableScanDesc,
            OldestXmin: TransactionId,
            liverows: *mut f64,
            deadrows: *mut f64,
            slot: *mut TupleTableSlot,
        ) -> bool,
    >,
    pub index_build_range_scan: ::std::option::Option<
        unsafe extern "C" fn(
            table_rel: Relation,
            index_rel: Relation,
            index_info: *mut IndexInfo,
            allow_sync: bool,
            anyvisible: bool,
            progress: bool,
            start_blockno: BlockNumber,
            numblocks: BlockNumber,
            callback: IndexBuildCallback,
            callback_state: *mut ::std::os::raw::c_void,
            scan: TableScanDesc,
        ) -> f64,
    >,
    pub index_validate_scan: ::std::option::Option<
        unsafe extern "C" fn(
            table_rel: Relation,
            index_rel: Relation,
            index_info: *mut IndexInfo,
            snapshot: Snapshot,
            state: *mut ValidateIndexState,
        ),
    >,
    pub relation_size: ::std::option::Option<
        unsafe extern "C" fn(rel: Relation, forkNumber: ForkNumber) -> uint64,
    >,
    pub relation_needs_toast_table:
        ::std::option::Option<unsafe extern "C" fn(rel: Relation) -> bool>,
    pub relation_estimate_size: ::std::option::Option<
        unsafe extern "C" fn(
            rel: Relation,
            attr_widths: *mut int32,
            pages: *mut BlockNumber,
            tuples: *mut f64,
            allvisfrac: *mut f64,
        ),
    >,
    pub scan_bitmap_next_block: ::std::option::Option<
        unsafe extern "C" fn(scan: TableScanDesc, tbmres: *mut TBMIterateResult) -> bool,
    >,
    pub scan_bitmap_next_tuple: ::std::option::Option<
        unsafe extern "C" fn(
            scan: TableScanDesc,
            tbmres: *mut TBMIterateResult,
            slot: *mut TupleTableSlot,
        ) -> bool,
    >,
    pub scan_sample_next_block: ::std::option::Option<
        unsafe extern "C" fn(scan: TableScanDesc, scanstate: *mut SampleScanState) -> bool,
    >,
    pub scan_sample_next_tuple: ::std::option::Option<
        unsafe extern "C" fn(
            scan: TableScanDesc,
            scanstate: *mut SampleScanState,
            slot: *mut TupleTableSlot,
        ) -> bool,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TableScanDescData {
    pub rs_rd: Relation,
    pub rs_snapshot: *mut SnapshotData,
    pub rs_nkeys: ::std::os::raw::c_int,
    pub rs_key: *mut ScanKeyData,
    pub rs_flags: uint32,
    pub rs_parallel: *mut ParallelTableScanDescData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TransactionStmt {
    pub type_: NodeTag,
    pub kind: TransactionStmtKind,
    pub options: *mut List,
    pub savepoint_name: *mut ::std::os::raw::c_char,
    pub gid: *mut ::std::os::raw::c_char,
    pub chain: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TransitionCaptureState {
    pub tcs_delete_old_table: bool,
    pub tcs_update_old_table: bool,
    pub tcs_update_new_table: bool,
    pub tcs_insert_new_table: bool,
    pub tcs_map: *mut TupleConversionMap,
    pub tcs_original_insert_tuple: *mut TupleTableSlot,
    pub tcs_private: *mut AfterTriggersTableData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TriggerData {
    pub type_: NodeTag,
    pub tg_event: TriggerEvent,
    pub tg_relation: Relation,
    pub tg_trigtuple: HeapTuple,
    pub tg_newtuple: HeapTuple,
    pub tg_trigger: *mut Trigger,
    pub tg_trigslot: *mut TupleTableSlot,
    pub tg_newslot: *mut TupleTableSlot,
    pub tg_oldtable: *mut Tuplestorestate,
    pub tg_newtable: *mut Tuplestorestate,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TupleConstr {
    pub defval: *mut AttrDefault,
    pub check: *mut ConstrCheck,
    pub missing: *mut AttrMissing,
    pub num_defval: uint16,
    pub num_check: uint16,
    pub has_not_null: bool,
    pub has_generated_stored: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TupleHashTableData {
    pub hashtab: *mut tuplehash_hash,
    pub numCols: ::std::os::raw::c_int,
    pub keyColIdx: *mut AttrNumber,
    pub tab_hash_funcs: *mut FmgrInfo,
    pub tab_eq_func: *mut ExprState,
    pub tab_collations: *mut Oid,
    pub tablecxt: MemoryContext,
    pub tempcxt: MemoryContext,
    pub entrysize: Size,
    pub tableslot: *mut TupleTableSlot,
    pub inputslot: *mut TupleTableSlot,
    pub in_hash_funcs: *mut FmgrInfo,
    pub cur_eq_func: *mut ExprState,
    pub hash_iv: uint32,
    pub exprcontext: *mut ExprContext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TupleTableSlot {
    pub type_: NodeTag,
    pub tts_flags: uint16,
    pub tts_nvalid: AttrNumber,
    pub tts_ops: *const TupleTableSlotOps,
    pub tts_tupleDescriptor: TupleDesc,
    pub tts_values: *mut Datum,
    pub tts_isnull: *mut bool,
    pub tts_mcxt: MemoryContext,
    pub tts_tid: ItemPointerData,
    pub tts_tableOid: Oid,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TuplesortInstrumentation {
    pub sortMethod: TuplesortMethod,
    pub spaceType: TuplesortSpaceType,
    pub spaceUsed: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TypeCacheEntry {
    pub type_id: Oid,
    pub typlen: int16,
    pub typbyval: bool,
    pub typalign: ::std::os::raw::c_char,
    pub typstorage: ::std::os::raw::c_char,
    pub typtype: ::std::os::raw::c_char,
    pub typrelid: Oid,
    pub typelem: Oid,
    pub typcollation: Oid,
    pub btree_opf: Oid,
    pub btree_opintype: Oid,
    pub hash_opf: Oid,
    pub hash_opintype: Oid,
    pub eq_opr: Oid,
    pub lt_opr: Oid,
    pub gt_opr: Oid,
    pub cmp_proc: Oid,
    pub hash_proc: Oid,
    pub hash_extended_proc: Oid,
    pub eq_opr_finfo: FmgrInfo,
    pub cmp_proc_finfo: FmgrInfo,
    pub hash_proc_finfo: FmgrInfo,
    pub hash_extended_proc_finfo: FmgrInfo,
    pub tupDesc: TupleDesc,
    pub tupDesc_identifier: uint64,
    pub rngelemtype: *mut TypeCacheEntry,
    pub rng_collation: Oid,
    pub rng_cmp_proc_finfo: FmgrInfo,
    pub rng_canonical_finfo: FmgrInfo,
    pub rng_subdiff_finfo: FmgrInfo,
    pub domainBaseType: Oid,
    pub domainBaseTypmod: int32,
    pub domainData: *mut DomainConstraintCache,
    pub flags: ::std::os::raw::c_int,
    pub enumData: *mut TypeCacheEnumData,
    pub nextDomain: *mut TypeCacheEntry,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Unique {
    pub plan: Plan,
    pub numCols: ::std::os::raw::c_int,
    pub uniqColIdx: *mut AttrNumber,
    pub uniqOperators: *mut Oid,
    pub uniqCollations: *mut Oid,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UniqueState {
    pub ps: PlanState,
    pub eqfunction: *mut ExprState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VacuumParams {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VacuumRelation {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    pub oid: Oid,
    pub va_cols: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VacuumStmt {
    pub type_: NodeTag,
    pub options: *mut List,
    pub rels: *mut List,
    pub is_vacuumcmd: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ValidateIndexState {
    pub tuplesort: *mut Tuplesortstate,
    pub htups: f64,
    pub itups: f64,
    pub tups_inserted: f64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VirtualTupleTableSlot {
    pub base: TupleTableSlot,
    pub data: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowAgg {
    pub plan: Plan,
    pub winref: Index,
    pub partNumCols: ::std::os::raw::c_int,
    pub partColIdx: *mut AttrNumber,
    pub partOperators: *mut Oid,
    pub partCollations: *mut Oid,
    pub ordNumCols: ::std::os::raw::c_int,
    pub ordColIdx: *mut AttrNumber,
    pub ordOperators: *mut Oid,
    pub ordCollations: *mut Oid,
    pub frameOptions: ::std::os::raw::c_int,
    pub startOffset: *mut Node,
    pub endOffset: *mut Node,
    pub startInRangeFunc: Oid,
    pub endInRangeFunc: Oid,
    pub inRangeColl: Oid,
    pub inRangeAsc: bool,
    pub inRangeNullsFirst: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowAggPath {
    pub path: Path,
    pub subpath: *mut Path,
    pub winclause: *mut WindowClause,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowAggState {
    pub ss: ScanState,
    pub funcs: *mut List,
    pub numfuncs: ::std::os::raw::c_int,
    pub numaggs: ::std::os::raw::c_int,
    pub perfunc: WindowStatePerFunc,
    pub peragg: WindowStatePerAgg,
    pub partEqfunction: *mut ExprState,
    pub ordEqfunction: *mut ExprState,
    pub buffer: *mut Tuplestorestate,
    pub current_ptr: ::std::os::raw::c_int,
    pub framehead_ptr: ::std::os::raw::c_int,
    pub frametail_ptr: ::std::os::raw::c_int,
    pub grouptail_ptr: ::std::os::raw::c_int,
    pub spooled_rows: int64,
    pub currentpos: int64,
    pub frameheadpos: int64,
    pub frametailpos: int64,
    pub agg_winobj: *mut WindowObjectData,
    pub aggregatedbase: int64,
    pub aggregatedupto: int64,
    pub frameOptions: ::std::os::raw::c_int,
    pub startOffset: *mut ExprState,
    pub endOffset: *mut ExprState,
    pub startOffsetValue: Datum,
    pub endOffsetValue: Datum,
    pub startInRangeFunc: FmgrInfo,
    pub endInRangeFunc: FmgrInfo,
    pub inRangeColl: Oid,
    pub inRangeAsc: bool,
    pub inRangeNullsFirst: bool,
    pub currentgroup: int64,
    pub frameheadgroup: int64,
    pub frametailgroup: int64,
    pub groupheadpos: int64,
    pub grouptailpos: int64,
    pub partcontext: MemoryContext,
    pub aggcontext: MemoryContext,
    pub curaggcontext: MemoryContext,
    pub tmpcontext: *mut ExprContext,
    pub all_first: bool,
    pub all_done: bool,
    pub partition_spooled: bool,
    pub more_partitions: bool,
    pub framehead_valid: bool,
    pub frametail_valid: bool,
    pub grouptail_valid: bool,
    pub first_part_slot: *mut TupleTableSlot,
    pub framehead_slot: *mut TupleTableSlot,
    pub frametail_slot: *mut TupleTableSlot,
    pub agg_row_slot: *mut TupleTableSlot,
    pub temp_slot_1: *mut TupleTableSlot,
    pub temp_slot_2: *mut TupleTableSlot,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowClause {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    pub refname: *mut ::std::os::raw::c_char,
    pub partitionClause: *mut List,
    pub orderClause: *mut List,
    pub frameOptions: ::std::os::raw::c_int,
    pub startOffset: *mut Node,
    pub endOffset: *mut Node,
    pub startInRangeFunc: Oid,
    pub endInRangeFunc: Oid,
    pub inRangeColl: Oid,
    pub inRangeAsc: bool,
    pub inRangeNullsFirst: bool,
    pub winref: Index,
    pub copiedOrder: bool,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct FormData_pg_index {
    pub indexrelid: Oid,
    pub indrelid: Oid,
    pub indnatts: int16,
    pub indnkeyatts: int16,
    pub indisunique: bool,
    pub indisprimary: bool,
    pub indisexclusion: bool,
    pub indimmediate: bool,
    pub indisclustered: bool,
    pub indisvalid: bool,
    pub indcheckxmin: bool,
    pub indisready: bool,
    pub indislive: bool,
    pub indisreplident: bool,
    pub indkey: int2vector,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct SharedHashInfo {
    pub num_workers: ::std::os::raw::c_int,
    pub hinstrument: __IncompleteArrayField<HashInstrumentation>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AggClauseCosts {
    pub numAggs: ::std::os::raw::c_int,
    pub numOrderedAggs: ::std::os::raw::c_int,
    pub hasNonPartial: bool,
    pub hasNonSerial: bool,
    pub transCost: QualCost,
    pub finalCost: QualCost,
    pub transitionSpace: Size,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AttrMissing {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AutoVacOpts {
    pub enabled: bool,
    pub vacuum_threshold: ::std::os::raw::c_int,
    pub analyze_threshold: ::std::os::raw::c_int,
    pub vacuum_cost_limit: ::std::os::raw::c_int,
    pub freeze_min_age: ::std::os::raw::c_int,
    pub freeze_max_age: ::std::os::raw::c_int,
    pub freeze_table_age: ::std::os::raw::c_int,
    pub multixact_freeze_min_age: ::std::os::raw::c_int,
    pub multixact_freeze_max_age: ::std::os::raw::c_int,
    pub multixact_freeze_table_age: ::std::os::raw::c_int,
    pub log_min_duration: ::std::os::raw::c_int,
    pub vacuum_cost_delay: float8,
    pub vacuum_scale_factor: float8,
    pub analyze_scale_factor: float8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct FinalPathExtraData {
    pub limit_needed: bool,
    pub limit_tuples: f64,
    pub count_est: int64,
    pub offset_est: int64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct FullTransactionId {
    pub value: uint64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct HashInstrumentation {
    pub nbuckets: ::std::os::raw::c_int,
    pub nbuckets_original: ::std::os::raw::c_int,
    pub nbatch: ::std::os::raw::c_int,
    pub nbatch_original: ::std::os::raw::c_int,
    pub space_peak: usize,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Instrumentation {
    pub need_timer: bool,
    pub need_bufusage: bool,
    pub running: bool,
    pub starttime: instr_time,
    pub counter: instr_time,
    pub firsttuple: f64,
    pub tuplecount: f64,
    pub bufusage_start: BufferUsage,
    pub startup: f64,
    pub total: f64,
    pub ntuples: f64,
    pub ntuples2: f64,
    pub nloops: f64,
    pub nfiltered1: f64,
    pub nfiltered2: f64,
    pub bufusage: BufferUsage,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct JitContext {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct JitInstrumentation {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct JoinCostWorkspace {
    pub startup_cost: Cost,
    pub total_cost: Cost,
    pub run_cost: Cost,
    pub inner_run_cost: Cost,
    pub inner_rescan_run_cost: Cost,
    pub outer_rows: f64,
    pub inner_rows: f64,
    pub outer_skip_rows: f64,
    pub inner_skip_rows: f64,
    pub numbuckets: ::std::os::raw::c_int,
    pub numbatches: ::std::os::raw::c_int,
    pub inner_rows_total: f64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct MemoryContextMethods {
    pub alloc: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, size: Size) -> *mut ::std::os::raw::c_void,
    >,
    pub free_p: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, pointer: *mut ::std::os::raw::c_void),
    >,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            context: MemoryContext,
            pointer: *mut ::std::os::raw::c_void,
            size: Size,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub reset: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
    pub delete_context: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
    pub get_chunk_space: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, pointer: *mut ::std::os::raw::c_void) -> Size,
    >,
    pub is_empty: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext) -> bool>,
    pub stats: ::std::option::Option<
        unsafe extern "C" fn(
            context: MemoryContext,
            printfunc: MemoryStatsPrintFunc,
            passthru: *mut ::std::os::raw::c_void,
            totals: *mut MemoryContextCounters,
        ),
    >,
    pub check: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NullableDatum {
    pub value: Datum,
    pub isnull: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ParallelBlockTableScanDescData {
    pub base: ParallelTableScanDescData,
    pub phs_nblocks: BlockNumber,
    pub phs_mutex: slock_t,
    pub phs_startblock: BlockNumber,
    pub phs_nallocated: pg_atomic_uint64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ParallelTableScanDescData {
    pub phs_relid: Oid,
    pub phs_syncscan: bool,
    pub phs_snapshot_any: bool,
    pub phs_snapshot_off: Size,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PartitionTupleRouting {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PublicationActions {
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
    pub pubtruncate: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SharedFileSet {
    pub creator_pid: pid_t,
    pub number: uint32,
    pub mutex: slock_t,
    pub refcnt: ::std::os::raw::c_int,
    pub ntablespaces: ::std::os::raw::c_int,
    pub tablespaces: [Oid; 8usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct SharedJitInstrumentation {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StdRdOptions {
    pub vl_len_: int32,
    pub fillfactor: ::std::os::raw::c_int,
    pub vacuum_cleanup_index_scale_factor: float8,
    pub toast_tuple_target: ::std::os::raw::c_int,
    pub autovacuum: AutoVacOpts,
    pub user_catalog_table: bool,
    pub parallel_workers: ::std::os::raw::c_int,
    pub vacuum_index_cleanup: bool,
    pub vacuum_truncate: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TM_FailureData {
    pub ctid: ItemPointerData,
    pub xmax: TransactionId,
    pub cmax: CommandId,
    pub traversed: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TupleTableSlotOps {
    pub base_slot_size: usize,
    pub init: ::std::option::Option<unsafe extern "C" fn(slot: *mut TupleTableSlot)>,
    pub release: ::std::option::Option<unsafe extern "C" fn(slot: *mut TupleTableSlot)>,
    pub clear: ::std::option::Option<unsafe extern "C" fn(slot: *mut TupleTableSlot)>,
    pub getsomeattrs: ::std::option::Option<
        unsafe extern "C" fn(slot: *mut TupleTableSlot, natts: ::std::os::raw::c_int),
    >,
    pub getsysattr: ::std::option::Option<
        unsafe extern "C" fn(
            slot: *mut TupleTableSlot,
            attnum: ::std::os::raw::c_int,
            isnull: *mut bool,
        ) -> Datum,
    >,
    pub materialize: ::std::option::Option<unsafe extern "C" fn(slot: *mut TupleTableSlot)>,
    pub copyslot: ::std::option::Option<
        unsafe extern "C" fn(dstslot: *mut TupleTableSlot, srcslot: *mut TupleTableSlot),
    >,
    pub get_heap_tuple:
        ::std::option::Option<unsafe extern "C" fn(slot: *mut TupleTableSlot) -> HeapTuple>,
    pub get_minimal_tuple:
        ::std::option::Option<unsafe extern "C" fn(slot: *mut TupleTableSlot) -> MinimalTuple>,
    pub copy_heap_tuple:
        ::std::option::Option<unsafe extern "C" fn(slot: *mut TupleTableSlot) -> HeapTuple>,
    pub copy_minimal_tuple:
        ::std::option::Option<unsafe extern "C" fn(slot: *mut TupleTableSlot) -> MinimalTuple>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct VariableCacheData {
    pub nextOid: Oid,
    pub oidCount: uint32,
    pub nextFullXid: FullTransactionId,
    pub oldestXid: TransactionId,
    pub xidVacLimit: TransactionId,
    pub xidWarnLimit: TransactionId,
    pub xidStopLimit: TransactionId,
    pub xidWrapLimit: TransactionId,
    pub oldestXidDB: Oid,
    pub oldestCommitTsXid: TransactionId,
    pub newestCommitTsXid: TransactionId,
    pub latestCompletedXid: TransactionId,
    pub oldestClogXid: TransactionId,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct xl_clog_truncate {
    pub pageno: ::std::os::raw::c_int,
    pub oldestXact: TransactionId,
    pub oldestXactDb: Oid,
}
#[repr(C)]
pub struct FormData_pg_trigger {
    pub oid: Oid,
    pub tgrelid: Oid,
    pub tgname: NameData,
    pub tgfoid: Oid,
    pub tgtype: int16,
    pub tgenabled: ::std::os::raw::c_char,
    pub tgisinternal: bool,
    pub tgconstrrelid: Oid,
    pub tgconstrindid: Oid,
    pub tgconstraint: Oid,
    pub tgdeferrable: bool,
    pub tginitdeferred: bool,
    pub tgnargs: int16,
    pub tgattr: int2vector,
}
#[repr(C)]
pub struct TupleDescData {
    pub natts: ::std::os::raw::c_int,
    pub tdtypeid: Oid,
    pub tdtypmod: int32,
    pub tdrefcount: ::std::os::raw::c_int,
    pub constr: *mut TupleConstr,
    pub attrs: __IncompleteArrayField<FormData_pg_attribute>,
}
impl Default for AttrDefault {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for BufferHeapTupleTableSlot {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for CachedExpression {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for CallContext {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for CallStmt {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for ConstrCheck {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for FunctionCallInfoBaseData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for GroupPathExtraData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for GroupResultPath {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for HeapTupleTableSlot {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for IndexClause {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for IndexFetchTableData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for MinimalTupleTableSlot {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for OnConflictSetState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for ParallelWorkerContext {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionPruneInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionPruneStep {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionPruneStepCombine {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionPruneStepOp {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionSchemeData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionedRelPruneInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PgBackendGSSStatus {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PgStat_MsgChecksumFailure {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for SharedSortInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for SortCoordinateData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for SubscriptingRef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for SupportRequestCost {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for SupportRequestIndexCondition {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for SupportRequestRows {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for SupportRequestSelectivity {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for SupportRequestSimplify {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for TableAmRoutine {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for TableScanDescData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for TupleConstr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for TupleDescData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for TuplesortInstrumentation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for VacuumRelation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for ValidateIndexState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for VirtualTupleTableSlot {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub const ACLITEMARRAYOID: u32 = 1034;
pub const ACL_ALL_RIGHTS_SCHEMA: u32 = 768;
pub const ATTRIBUTE_GENERATED_STORED: u8 = 115u8;
pub const AlterTableType_AT_AddConstraint: AlterTableType = 15;
pub const AlterTableType_AT_AddConstraintRecurse: AlterTableType = 16;
pub const AlterTableType_AT_AddIdentity: AlterTableType = 63;
pub const AlterTableType_AT_AddIndex: AlterTableType = 13;
pub const AlterTableType_AT_AddIndexConstraint: AlterTableType = 23;
pub const AlterTableType_AT_AddInherit: AlterTableType = 51;
pub const AlterTableType_AT_AddOf: AlterTableType = 53;
pub const AlterTableType_AT_AlterColumnGenericOptions: AlterTableType = 28;
pub const AlterTableType_AT_AlterColumnType: AlterTableType = 27;
pub const AlterTableType_AT_AlterConstraint: AlterTableType = 19;
pub const AlterTableType_AT_AttachPartition: AlterTableType = 61;
pub const AlterTableType_AT_ChangeOwner: AlterTableType = 29;
pub const AlterTableType_AT_CheckNotNull: AlterTableType = 6;
pub const AlterTableType_AT_ClusterOn: AlterTableType = 30;
pub const AlterTableType_AT_DetachPartition: AlterTableType = 62;
pub const AlterTableType_AT_DisableRowSecurity: AlterTableType = 57;
pub const AlterTableType_AT_DisableRule: AlterTableType = 50;
pub const AlterTableType_AT_DisableTrig: AlterTableType = 42;
pub const AlterTableType_AT_DisableTrigAll: AlterTableType = 44;
pub const AlterTableType_AT_DisableTrigUser: AlterTableType = 46;
pub const AlterTableType_AT_DropCluster: AlterTableType = 31;
pub const AlterTableType_AT_DropColumn: AlterTableType = 11;
pub const AlterTableType_AT_DropColumnRecurse: AlterTableType = 12;
pub const AlterTableType_AT_DropConstraint: AlterTableType = 24;
pub const AlterTableType_AT_DropConstraintRecurse: AlterTableType = 25;
pub const AlterTableType_AT_DropIdentity: AlterTableType = 65;
pub const AlterTableType_AT_DropInherit: AlterTableType = 52;
pub const AlterTableType_AT_DropOf: AlterTableType = 54;
pub const AlterTableType_AT_DropOids: AlterTableType = 34;
pub const AlterTableType_AT_EnableAlwaysRule: AlterTableType = 48;
pub const AlterTableType_AT_EnableAlwaysTrig: AlterTableType = 40;
pub const AlterTableType_AT_EnableReplicaRule: AlterTableType = 49;
pub const AlterTableType_AT_EnableReplicaTrig: AlterTableType = 41;
pub const AlterTableType_AT_EnableRowSecurity: AlterTableType = 56;
pub const AlterTableType_AT_EnableRule: AlterTableType = 47;
pub const AlterTableType_AT_EnableTrig: AlterTableType = 39;
pub const AlterTableType_AT_EnableTrigAll: AlterTableType = 43;
pub const AlterTableType_AT_EnableTrigUser: AlterTableType = 45;
pub const AlterTableType_AT_ForceRowSecurity: AlterTableType = 58;
pub const AlterTableType_AT_GenericOptions: AlterTableType = 60;
pub const AlterTableType_AT_NoForceRowSecurity: AlterTableType = 59;
pub const AlterTableType_AT_ProcessedConstraint: AlterTableType = 22;
pub const AlterTableType_AT_ReAddComment: AlterTableType = 26;
pub const AlterTableType_AT_ReAddConstraint: AlterTableType = 17;
pub const AlterTableType_AT_ReAddDomainConstraint: AlterTableType = 18;
pub const AlterTableType_AT_ReAddIndex: AlterTableType = 14;
pub const AlterTableType_AT_ReplaceRelOptions: AlterTableType = 38;
pub const AlterTableType_AT_ReplicaIdentity: AlterTableType = 55;
pub const AlterTableType_AT_ResetOptions: AlterTableType = 9;
pub const AlterTableType_AT_ResetRelOptions: AlterTableType = 37;
pub const AlterTableType_AT_SetIdentity: AlterTableType = 64;
pub const AlterTableType_AT_SetLogged: AlterTableType = 32;
pub const AlterTableType_AT_SetOptions: AlterTableType = 8;
pub const AlterTableType_AT_SetRelOptions: AlterTableType = 36;
pub const AlterTableType_AT_SetStatistics: AlterTableType = 7;
pub const AlterTableType_AT_SetStorage: AlterTableType = 10;
pub const AlterTableType_AT_SetTableSpace: AlterTableType = 35;
pub const AlterTableType_AT_SetUnLogged: AlterTableType = 33;
pub const AlterTableType_AT_ValidateConstraint: AlterTableType = 20;
pub const AlterTableType_AT_ValidateConstraintRecurse: AlterTableType = 21;
pub const Anum_pg_attribute_attacl: u32 = 22;
pub const Anum_pg_attribute_attcollation: u32 = 21;
pub const Anum_pg_attribute_attfdwoptions: u32 = 24;
pub const Anum_pg_attribute_attgenerated: u32 = 17;
pub const Anum_pg_attribute_atthasmissing: u32 = 15;
pub const Anum_pg_attribute_attidentity: u32 = 16;
pub const Anum_pg_attribute_attinhcount: u32 = 20;
pub const Anum_pg_attribute_attisdropped: u32 = 18;
pub const Anum_pg_attribute_attislocal: u32 = 19;
pub const Anum_pg_attribute_attmissingval: u32 = 25;
pub const Anum_pg_attribute_attoptions: u32 = 23;
pub const Anum_pg_class_oid: u32 = 1;
pub const Anum_pg_class_relallvisible: u32 = 12;
pub const Anum_pg_class_relam: u32 = 7;
pub const Anum_pg_class_relchecks: u32 = 19;
pub const Anum_pg_class_relfilenode: u32 = 8;
pub const Anum_pg_class_relforcerowsecurity: u32 = 24;
pub const Anum_pg_class_relhasindex: u32 = 14;
pub const Anum_pg_class_relhasrules: u32 = 20;
pub const Anum_pg_class_relhassubclass: u32 = 22;
pub const Anum_pg_class_relhastriggers: u32 = 21;
pub const Anum_pg_class_relispartition: u32 = 27;
pub const Anum_pg_class_relispopulated: u32 = 25;
pub const Anum_pg_class_relisshared: u32 = 15;
pub const Anum_pg_class_relkind: u32 = 17;
pub const Anum_pg_class_relname: u32 = 2;
pub const Anum_pg_class_relnamespace: u32 = 3;
pub const Anum_pg_class_relnatts: u32 = 18;
pub const Anum_pg_class_reloftype: u32 = 5;
pub const Anum_pg_class_relowner: u32 = 6;
pub const Anum_pg_class_relpages: u32 = 10;
pub const Anum_pg_class_relpersistence: u32 = 16;
pub const Anum_pg_class_relreplident: u32 = 26;
pub const Anum_pg_class_relrewrite: u32 = 28;
pub const Anum_pg_class_relrowsecurity: u32 = 23;
pub const Anum_pg_class_reltablespace: u32 = 9;
pub const Anum_pg_class_reltoastrelid: u32 = 13;
pub const Anum_pg_class_reltuples: u32 = 11;
pub const Anum_pg_class_reltype: u32 = 4;
pub const Anum_pg_enum_enumlabel: u32 = 4;
pub const Anum_pg_enum_enumsortorder: u32 = 3;
pub const Anum_pg_enum_enumtypid: u32 = 2;
pub const Anum_pg_enum_oid: u32 = 1;
pub const Anum_pg_event_trigger_evtenabled: u32 = 6;
pub const Anum_pg_event_trigger_evtevent: u32 = 3;
pub const Anum_pg_event_trigger_evtfoid: u32 = 5;
pub const Anum_pg_event_trigger_evtname: u32 = 2;
pub const Anum_pg_event_trigger_evtowner: u32 = 4;
pub const Anum_pg_event_trigger_evttags: u32 = 7;
pub const Anum_pg_event_trigger_oid: u32 = 1;
pub const Anum_pg_index_indcheckxmin: u32 = 11;
pub const Anum_pg_index_indclass: u32 = 17;
pub const Anum_pg_index_indcollation: u32 = 16;
pub const Anum_pg_index_indexprs: u32 = 19;
pub const Anum_pg_index_indimmediate: u32 = 8;
pub const Anum_pg_index_indisclustered: u32 = 9;
pub const Anum_pg_index_indisexclusion: u32 = 7;
pub const Anum_pg_index_indislive: u32 = 13;
pub const Anum_pg_index_indisprimary: u32 = 6;
pub const Anum_pg_index_indisready: u32 = 12;
pub const Anum_pg_index_indisreplident: u32 = 14;
pub const Anum_pg_index_indisunique: u32 = 5;
pub const Anum_pg_index_indisvalid: u32 = 10;
pub const Anum_pg_index_indkey: u32 = 15;
pub const Anum_pg_index_indnkeyatts: u32 = 4;
pub const Anum_pg_index_indoption: u32 = 18;
pub const Anum_pg_index_indpred: u32 = 20;
pub const Anum_pg_publication_oid: u32 = 1;
pub const Anum_pg_publication_puballtables: u32 = 4;
pub const Anum_pg_publication_pubdelete: u32 = 7;
pub const Anum_pg_publication_pubinsert: u32 = 5;
pub const Anum_pg_publication_pubname: u32 = 2;
pub const Anum_pg_publication_pubowner: u32 = 3;
pub const Anum_pg_publication_pubtruncate: u32 = 8;
pub const Anum_pg_publication_pubupdate: u32 = 6;
pub const Anum_pg_trigger_oid: u32 = 1;
pub const Anum_pg_trigger_tgargs: u32 = 15;
pub const Anum_pg_trigger_tgattr: u32 = 14;
pub const Anum_pg_trigger_tgconstraint: u32 = 10;
pub const Anum_pg_trigger_tgconstrindid: u32 = 9;
pub const Anum_pg_trigger_tgconstrrelid: u32 = 8;
pub const Anum_pg_trigger_tgdeferrable: u32 = 11;
pub const Anum_pg_trigger_tgenabled: u32 = 6;
pub const Anum_pg_trigger_tgfoid: u32 = 4;
pub const Anum_pg_trigger_tginitdeferred: u32 = 12;
pub const Anum_pg_trigger_tgisinternal: u32 = 7;
pub const Anum_pg_trigger_tgname: u32 = 3;
pub const Anum_pg_trigger_tgnargs: u32 = 13;
pub const Anum_pg_trigger_tgnewtable: u32 = 18;
pub const Anum_pg_trigger_tgoldtable: u32 = 17;
pub const Anum_pg_trigger_tgqual: u32 = 16;
pub const Anum_pg_trigger_tgrelid: u32 = 2;
pub const Anum_pg_trigger_tgtype: u32 = 5;
pub const Anum_pg_type_oid: u32 = 1;
pub const Anum_pg_type_typacl: u32 = 31;
pub const Anum_pg_type_typalign: u32 = 22;
pub const Anum_pg_type_typanalyze: u32 = 21;
pub const Anum_pg_type_typarray: u32 = 14;
pub const Anum_pg_type_typbasetype: u32 = 25;
pub const Anum_pg_type_typbyval: u32 = 6;
pub const Anum_pg_type_typcategory: u32 = 8;
pub const Anum_pg_type_typcollation: u32 = 28;
pub const Anum_pg_type_typdefault: u32 = 30;
pub const Anum_pg_type_typdefaultbin: u32 = 29;
pub const Anum_pg_type_typdelim: u32 = 11;
pub const Anum_pg_type_typelem: u32 = 13;
pub const Anum_pg_type_typinput: u32 = 15;
pub const Anum_pg_type_typisdefined: u32 = 10;
pub const Anum_pg_type_typispreferred: u32 = 9;
pub const Anum_pg_type_typlen: u32 = 5;
pub const Anum_pg_type_typmodin: u32 = 19;
pub const Anum_pg_type_typmodout: u32 = 20;
pub const Anum_pg_type_typname: u32 = 2;
pub const Anum_pg_type_typnamespace: u32 = 3;
pub const Anum_pg_type_typndims: u32 = 27;
pub const Anum_pg_type_typnotnull: u32 = 24;
pub const Anum_pg_type_typoutput: u32 = 16;
pub const Anum_pg_type_typowner: u32 = 4;
pub const Anum_pg_type_typreceive: u32 = 17;
pub const Anum_pg_type_typrelid: u32 = 12;
pub const Anum_pg_type_typsend: u32 = 18;
pub const Anum_pg_type_typstorage: u32 = 23;
pub const Anum_pg_type_typtype: u32 = 7;
pub const Anum_pg_type_typtypmod: u32 = 26;
pub const BGWORKER_BYPASS_ALLOWCONN: u32 = 1;
pub const BGW_MAXLEN: u32 = 96;
pub const BITARRAYOID: u32 = 1561;
pub const BITS_PER_BITMAPWORD: u32 = 64;
pub const BOOLARRAYOID: u32 = 1000;
pub const BOXARRAYOID: u32 = 1020;
pub const BPCHARARRAYOID: u32 = 1014;
pub const BUFFER_MAPPING_LWLOCK_OFFSET: u32 = 45;
pub const BYTEAARRAYOID: u32 = 1001;
pub const BuiltinTrancheIds_LWTRANCHE_ASYNC_BUFFERS: BuiltinTrancheIds = 50;
pub const BuiltinTrancheIds_LWTRANCHE_BUFFER_CONTENT: BuiltinTrancheIds = 53;
pub const BuiltinTrancheIds_LWTRANCHE_BUFFER_IO_IN_PROGRESS: BuiltinTrancheIds = 54;
pub const BuiltinTrancheIds_LWTRANCHE_BUFFER_MAPPING: BuiltinTrancheIds = 58;
pub const BuiltinTrancheIds_LWTRANCHE_CLOG_BUFFERS: BuiltinTrancheIds = 45;
pub const BuiltinTrancheIds_LWTRANCHE_COMMITTS_BUFFERS: BuiltinTrancheIds = 46;
pub const BuiltinTrancheIds_LWTRANCHE_FIRST_USER_DEFINED: BuiltinTrancheIds = 70;
pub const BuiltinTrancheIds_LWTRANCHE_LOCK_MANAGER: BuiltinTrancheIds = 59;
pub const BuiltinTrancheIds_LWTRANCHE_MXACTMEMBER_BUFFERS: BuiltinTrancheIds = 49;
pub const BuiltinTrancheIds_LWTRANCHE_MXACTOFFSET_BUFFERS: BuiltinTrancheIds = 48;
pub const BuiltinTrancheIds_LWTRANCHE_OLDSERXID_BUFFERS: BuiltinTrancheIds = 51;
pub const BuiltinTrancheIds_LWTRANCHE_PARALLEL_APPEND: BuiltinTrancheIds = 68;
pub const BuiltinTrancheIds_LWTRANCHE_PARALLEL_HASH_JOIN: BuiltinTrancheIds = 61;
pub const BuiltinTrancheIds_LWTRANCHE_PARALLEL_QUERY_DSA: BuiltinTrancheIds = 62;
pub const BuiltinTrancheIds_LWTRANCHE_PREDICATE_LOCK_MANAGER: BuiltinTrancheIds = 60;
pub const BuiltinTrancheIds_LWTRANCHE_PROC: BuiltinTrancheIds = 57;
pub const BuiltinTrancheIds_LWTRANCHE_REPLICATION_ORIGIN: BuiltinTrancheIds = 55;
pub const BuiltinTrancheIds_LWTRANCHE_REPLICATION_SLOT_IO_IN_PROGRESS: BuiltinTrancheIds = 56;
pub const BuiltinTrancheIds_LWTRANCHE_SESSION_DSA: BuiltinTrancheIds = 63;
pub const BuiltinTrancheIds_LWTRANCHE_SESSION_RECORD_TABLE: BuiltinTrancheIds = 64;
pub const BuiltinTrancheIds_LWTRANCHE_SESSION_TYPMOD_TABLE: BuiltinTrancheIds = 65;
pub const BuiltinTrancheIds_LWTRANCHE_SHARED_TUPLESTORE: BuiltinTrancheIds = 66;
pub const BuiltinTrancheIds_LWTRANCHE_SUBTRANS_BUFFERS: BuiltinTrancheIds = 47;
pub const BuiltinTrancheIds_LWTRANCHE_SXACT: BuiltinTrancheIds = 69;
pub const BuiltinTrancheIds_LWTRANCHE_TBM: BuiltinTrancheIds = 67;
pub const BuiltinTrancheIds_LWTRANCHE_WAL_INSERT: BuiltinTrancheIds = 52;
pub const CACHEDEXPR_MAGIC: u32 = 838275847;
pub const CATALOG_VERSION_NO: u32 = 201909212;
pub const CHARARRAYOID: u32 = 1002;
pub const CHECKPOINT_CAUSE_TIME: u32 = 256;
pub const CHECKPOINT_CAUSE_XLOG: u32 = 128;
pub const CHECKPOINT_REQUESTED: u32 = 64;
pub const CIDARRAYOID: u32 = 1012;
pub const CIDRARRAYOID: u32 = 651;
pub const CIRCLEARRAYOID: u32 = 719;
pub const CLOG_TRUNCATE: u32 = 16;
pub const CLOG_ZEROPAGE: u32 = 0;
pub const CTEMaterialize_CTEMaterializeAlways: CTEMaterialize = 1;
pub const CTEMaterialize_CTEMaterializeDefault: CTEMaterialize = 0;
pub const CTEMaterialize_CTEMaterializeNever: CTEMaterialize = 2;
pub const ClusterOption_CLUOPT_RECHECK: ClusterOption = 1;
pub const ClusterOption_CLUOPT_VERBOSE: ClusterOption = 2;
pub const ConstrType_CONSTR_ATTR_DEFERRABLE: ConstrType = 10;
pub const ConstrType_CONSTR_ATTR_DEFERRED: ConstrType = 12;
pub const ConstrType_CONSTR_ATTR_IMMEDIATE: ConstrType = 13;
pub const ConstrType_CONSTR_ATTR_NOT_DEFERRABLE: ConstrType = 11;
pub const ConstrType_CONSTR_CHECK: ConstrType = 5;
pub const ConstrType_CONSTR_EXCLUSION: ConstrType = 8;
pub const ConstrType_CONSTR_FOREIGN: ConstrType = 9;
pub const ConstrType_CONSTR_GENERATED: ConstrType = 4;
pub const ConstrType_CONSTR_PRIMARY: ConstrType = 6;
pub const ConstrType_CONSTR_UNIQUE: ConstrType = 7;
pub const DATEARRAYOID: u32 = 1182;
pub const DATERANGEARRAYOID: u32 = 3913;
pub const DATERANGEOID: u32 = 3912;
pub const DEFAULT_TABLE_ACCESS_METHOD: &'static [u8; 5usize] = b"heap\0";
pub const DEFAULT_XLOG_SEG_SIZE: u32 = 16777216;
pub const DEF_PGPORT: u32 = 28812;
pub const DEF_PGPORT_STR: &'static [u8; 6usize] = b"28812\0";
pub const DependencyType_DEPENDENCY_PARTITION_PRI: DependencyType = 80;
pub const DependencyType_DEPENDENCY_PARTITION_SEC: DependencyType = 83;
pub const EXEC_FLAG_WITH_NO_DATA: u32 = 32;
pub const FALLBACK_PROMOTE_SIGNAL_FILE: &'static [u8; 17usize] = b"fallback_promote\0";
pub const FDW_MISSING_OK: u32 = 1;
pub const FIELDNO_AGGSTATE_ALL_PERGROUPS: u32 = 34;
pub const FIELDNO_AGGSTATE_CURAGGCONTEXT: u32 = 14;
pub const FIELDNO_AGGSTATE_CURPERTRANS: u32 = 16;
pub const FIELDNO_AGGSTATE_CURRENT_SET: u32 = 20;
pub const FIELDNO_EXPRCONTEXT_AGGNULLS: u32 = 9;
pub const FIELDNO_EXPRCONTEXT_AGGVALUES: u32 = 8;
pub const FIELDNO_EXPRCONTEXT_CASEDATUM: u32 = 10;
pub const FIELDNO_EXPRCONTEXT_CASENULL: u32 = 11;
pub const FIELDNO_EXPRCONTEXT_DOMAINDATUM: u32 = 12;
pub const FIELDNO_EXPRCONTEXT_DOMAINNULL: u32 = 13;
pub const FIELDNO_EXPRCONTEXT_INNERTUPLE: u32 = 2;
pub const FIELDNO_EXPRCONTEXT_OUTERTUPLE: u32 = 3;
pub const FIELDNO_EXPRCONTEXT_SCANTUPLE: u32 = 1;
pub const FIELDNO_EXPRSTATE_RESNULL: u32 = 2;
pub const FIELDNO_EXPRSTATE_RESULTSLOT: u32 = 4;
pub const FIELDNO_EXPRSTATE_RESVALUE: u32 = 3;
pub const FIELDNO_FUNCTIONCALLINFODATA_ARGS: u32 = 6;
pub const FIELDNO_FUNCTIONCALLINFODATA_ISNULL: u32 = 4;
pub const FIELDNO_HEAPTUPLEDATA_DATA: u32 = 3;
pub const FIELDNO_HEAPTUPLEHEADERDATA_BITS: u32 = 5;
pub const FIELDNO_HEAPTUPLEHEADERDATA_HOFF: u32 = 4;
pub const FIELDNO_HEAPTUPLEHEADERDATA_INFOMASK: u32 = 3;
pub const FIELDNO_HEAPTUPLEHEADERDATA_INFOMASK2: u32 = 2;
pub const FIELDNO_HEAPTUPLETABLESLOT_OFF: u32 = 2;
pub const FIELDNO_HEAPTUPLETABLESLOT_TUPLE: u32 = 1;
pub const FIELDNO_MINIMALTUPLETABLESLOT_OFF: u32 = 4;
pub const FIELDNO_MINIMALTUPLETABLESLOT_TUPLE: u32 = 1;
pub const FIELDNO_NULLABLE_DATUM_DATUM: u32 = 0;
pub const FIELDNO_NULLABLE_DATUM_ISNULL: u32 = 1;
pub const FIELDNO_TUPLETABLESLOT_FLAGS: u32 = 1;
pub const FIELDNO_TUPLETABLESLOT_ISNULL: u32 = 6;
pub const FIELDNO_TUPLETABLESLOT_NVALID: u32 = 2;
pub const FIELDNO_TUPLETABLESLOT_TUPLEDESCRIPTOR: u32 = 4;
pub const FIELDNO_TUPLETABLESLOT_VALUES: u32 = 5;
pub const FLOAT8ARRAYOID: u32 = 1022;
pub const FORMAT_TYPE_ALLOW_INVALID: u32 = 2;
pub const FORMAT_TYPE_FORCE_QUALIFY: u32 = 4;
pub const FORMAT_TYPE_TYPEMOD_GIVEN: u32 = 1;
pub const FRAMEOPTION_BETWEEN: u32 = 16;
pub const FRAMEOPTION_DEFAULTS: u32 = 1058;
pub const FRAMEOPTION_END_CURRENT_ROW: u32 = 1024;
pub const FRAMEOPTION_END_OFFSET: u32 = 20480;
pub const FRAMEOPTION_END_OFFSET_FOLLOWING: u32 = 16384;
pub const FRAMEOPTION_END_OFFSET_PRECEDING: u32 = 4096;
pub const FRAMEOPTION_END_UNBOUNDED_FOLLOWING: u32 = 256;
pub const FRAMEOPTION_END_UNBOUNDED_PRECEDING: u32 = 64;
pub const FRAMEOPTION_EXCLUDE_CURRENT_ROW: u32 = 32768;
pub const FRAMEOPTION_EXCLUDE_GROUP: u32 = 65536;
pub const FRAMEOPTION_EXCLUDE_TIES: u32 = 131072;
pub const FRAMEOPTION_EXCLUSION: u32 = 229376;
pub const FRAMEOPTION_GROUPS: u32 = 8;
pub const FRAMEOPTION_START_CURRENT_ROW: u32 = 512;
pub const FRAMEOPTION_START_OFFSET: u32 = 10240;
pub const FRAMEOPTION_START_OFFSET_FOLLOWING: u32 = 8192;
pub const FRAMEOPTION_START_OFFSET_PRECEDING: u32 = 2048;
pub const FRAMEOPTION_START_UNBOUNDED_FOLLOWING: u32 = 128;
pub const FRAMEOPTION_START_UNBOUNDED_PRECEDING: u32 = 32;
pub const FSV_MISSING_OK: u32 = 1;
pub const FirstBootstrapObjectId: u32 = 12000;
pub const FirstGenbkiObjectId: u32 = 10000;
pub const FirstLowInvalidHeapAttributeNumber: i32 = -7;
pub const ForceParallelMode_FORCE_PARALLEL_OFF: ForceParallelMode = 0;
pub const ForceParallelMode_FORCE_PARALLEL_ON: ForceParallelMode = 1;
pub const ForceParallelMode_FORCE_PARALLEL_REGRESS: ForceParallelMode = 2;
pub const FuncDetailCode_FUNCDETAIL_AGGREGATE: FuncDetailCode = 4;
pub const FuncDetailCode_FUNCDETAIL_COERCION: FuncDetailCode = 6;
pub const FuncDetailCode_FUNCDETAIL_PROCEDURE: FuncDetailCode = 3;
pub const FuncDetailCode_FUNCDETAIL_WINDOWFUNC: FuncDetailCode = 5;
pub const GIDSIZE: u32 = 200;
pub const GROUPING_CAN_PARTIAL_AGG: u32 = 4;
pub const GROUPING_CAN_USE_HASH: u32 = 2;
pub const GROUPING_CAN_USE_SORT: u32 = 1;
pub const GTSVECTORARRAYOID: u32 = 3644;
pub const GUC_EXPLAIN: u32 = 1048576;
pub const GUC_UNIT_BYTE: u32 = 32768;
pub const HAVE_COPYFILE: u32 = 1;
pub const HAVE_COPYFILE_H: u32 = 1;
pub const HAVE_DECL_RTLD_GLOBAL: u32 = 1;
pub const HAVE_DECL_RTLD_NOW: u32 = 1;
pub const HAVE_DECL_STRNLEN: u32 = 1;
pub const HAVE_INDEXOPTINFO_TYPEDEF: u32 = 1;
pub const HAVE_PLANNERINFO_TYPEDEF: u32 = 1;
pub const HAVE_PREAD: u32 = 1;
pub const HAVE_PWRITE: u32 = 1;
pub const HAVE_SPECIALJOININFO_TYPEDEF: u32 = 1;
pub const HAVE_STDBOOL_H: u32 = 1;
pub const HAVE_STRNLEN: u32 = 1;
pub const HAVE_STRSIGNAL: u32 = 1;
pub const HAVE_STRTOF: u32 = 1;
pub const HAVE_X86_64_POPCNTQ: u32 = 1;
pub const HAVE__BOOL: u32 = 1;
pub const HAVE__BUILTIN_BSWAP16: u32 = 1;
pub const HAVE__BUILTIN_CLZ: u32 = 1;
pub const HAVE__BUILTIN_CTZ: u32 = 1;
pub const HAVE__BUILTIN_OP_OVERFLOW: u32 = 1;
pub const HAVE__BUILTIN_POPCOUNT: u32 = 1;
pub const HEAP_HASOID_OLD: u32 = 8;
pub const HTEqualStrategyNumber: u32 = 1;
pub const HTMaxStrategyNumber: u32 = 1;
pub const INDEX_AM_RESERVED_BIT: u32 = 8192;
pub const INDEX_CONSTR_CREATE_DEFERRABLE: u32 = 2;
pub const INDEX_CONSTR_CREATE_INIT_DEFERRED: u32 = 4;
pub const INDEX_CONSTR_CREATE_MARK_AS_PRIMARY: u32 = 1;
pub const INDEX_CONSTR_CREATE_REMOVE_OLD_DEPS: u32 = 16;
pub const INDEX_CONSTR_CREATE_UPDATE_INDEX: u32 = 8;
pub const INDEX_CREATE_ADD_CONSTRAINT: u32 = 2;
pub const INDEX_CREATE_CONCURRENT: u32 = 8;
pub const INDEX_CREATE_IF_NOT_EXISTS: u32 = 16;
pub const INDEX_CREATE_INVALID: u32 = 64;
pub const INDEX_CREATE_IS_PRIMARY: u32 = 1;
pub const INDEX_CREATE_PARTITIONED: u32 = 32;
pub const INDEX_CREATE_SKIP_BUILD: u32 = 4;
pub const INETARRAYOID: u32 = 1041;
pub const INT2VECTORARRAYOID: u32 = 1006;
pub const INT4RANGEARRAYOID: u32 = 3905;
pub const INT8ARRAYOID: u32 = 1016;
pub const INT8RANGEARRAYOID: u32 = 3927;
pub const INT8RANGEOID: u32 = 3926;
pub const INTERVALARRAYOID: u32 = 1187;
pub const IndexAMProperty_AMPROP_CAN_INCLUDE: IndexAMProperty = 18;
pub const IndexAttrBitmapKind_INDEX_ATTR_BITMAP_ALL: IndexAttrBitmapKind = 0;
pub const IndexAttrBitmapKind_INDEX_ATTR_BITMAP_IDENTITY_KEY: IndexAttrBitmapKind = 3;
pub const IndexAttrBitmapKind_INDEX_ATTR_BITMAP_KEY: IndexAttrBitmapKind = 1;
pub const IndexAttrBitmapKind_INDEX_ATTR_BITMAP_PRIMARY_KEY: IndexAttrBitmapKind = 2;
pub const InheritanceKind_INHKIND_INHERITED: InheritanceKind = 1;
pub const InheritanceKind_INHKIND_NONE: InheritanceKind = 0;
pub const InheritanceKind_INHKIND_PARTITIONED: InheritanceKind = 2;
pub const JSONARRAYOID: u32 = 199;
pub const JSONBARRAYOID: u32 = 3807;
pub const JSONPATHARRAYOID: u32 = 4073;
pub const JSONPATHOID: u32 = 4072;
pub const JsonbJsonpathExistsStrategyNumber: u32 = 15;
pub const JsonbJsonpathPredicateStrategyNumber: u32 = 16;
pub const LINEARRAYOID: u32 = 629;
pub const LOCK_MANAGER_LWLOCK_OFFSET: u32 = 173;
pub const LSEGARRAYOID: u32 = 1018;
pub const MACADDR8ARRAYOID: u32 = 775;
pub const MACADDRARRAYOID: u32 = 1040;
pub const MONEYARRAYOID: u32 = 791;
pub const MaxCommandIdAttributeNumber: i32 = -5;
pub const MaxLockMode: u32 = 8;
pub const MaxTransactionIdAttributeNumber: i32 = -4;
pub const MinCommandIdAttributeNumber: i32 = -3;
pub const MinTransactionIdAttributeNumber: i32 = -2;
pub const MovedPartitionsOffsetNumber: u32 = 65533;
pub const NAMEARRAYOID: u32 = 1003;
pub const NUMERICARRAYOID: u32 = 1231;
pub const NUMRANGEARRAYOID: u32 = 3907;
pub const NUMRANGEOID: u32 = 3906;
pub const NUM_FIXED_LWLOCKS: u32 = 205;
pub const NUM_INDIVIDUAL_LWLOCKS: u32 = 45;
pub const Natts_pg_attribute: u32 = 25;
pub const Natts_pg_enum: u32 = 4;
pub const Natts_pg_event_trigger: u32 = 7;
pub const Natts_pg_index: u32 = 20;
pub const Natts_pg_publication: u32 = 8;
pub const Natts_pg_trigger: u32 = 18;
pub const Natts_pg_type: u32 = 31;
pub const NodeTag_T_A_ArrayExpr: NodeTag = 349;
pub const NodeTag_T_A_Const: NodeTag = 344;
pub const NodeTag_T_A_Expr: NodeTag = 341;
pub const NodeTag_T_A_Indices: NodeTag = 347;
pub const NodeTag_T_A_Indirection: NodeTag = 348;
pub const NodeTag_T_A_Star: NodeTag = 346;
pub const NodeTag_T_AccessPriv: NodeTag = 374;
pub const NodeTag_T_Agg: NodeTag = 42;
pub const NodeTag_T_AggPath: NodeTag = 189;
pub const NodeTag_T_AggState: NodeTag = 91;
pub const NodeTag_T_Aggref: NodeTag = 107;
pub const NodeTag_T_AggrefExprState: NodeTag = 153;
pub const NodeTag_T_Alias: NodeTag = 100;
pub const NodeTag_T_AllocSetContext: NodeTag = 214;
pub const NodeTag_T_AlterCollationStmt: NodeTag = 339;
pub const NodeTag_T_AlterDatabaseSetStmt: NodeTag = 284;
pub const NodeTag_T_AlterDatabaseStmt: NodeTag = 283;
pub const NodeTag_T_AlterDefaultPrivilegesStmt: NodeTag = 240;
pub const NodeTag_T_AlterDomainStmt: NodeTag = 236;
pub const NodeTag_T_AlterEnumStmt: NodeTag = 306;
pub const NodeTag_T_AlterEventTrigStmt: NodeTag = 325;
pub const NodeTag_T_AlterExtensionContentsStmt: NodeTag = 323;
pub const NodeTag_T_AlterExtensionStmt: NodeTag = 322;
pub const NodeTag_T_AlterFdwStmt: NodeTag = 310;
pub const NodeTag_T_AlterForeignServerStmt: NodeTag = 312;
pub const NodeTag_T_AlterFunctionStmt: NodeTag = 252;
pub const NodeTag_T_AlterObjectDependsStmt: NodeTag = 297;
pub const NodeTag_T_AlterObjectSchemaStmt: NodeTag = 298;
pub const NodeTag_T_AlterOpFamilyStmt: NodeTag = 290;
pub const NodeTag_T_AlterOperatorStmt: NodeTag = 300;
pub const NodeTag_T_AlterOwnerStmt: NodeTag = 299;
pub const NodeTag_T_AlterPolicyStmt: NodeTag = 330;
pub const NodeTag_T_AlterPublicationStmt: NodeTag = 334;
pub const NodeTag_T_AlterRoleSetStmt: NodeTag = 285;
pub const NodeTag_T_AlterRoleStmt: NodeTag = 276;
pub const NodeTag_T_AlterSeqStmt: NodeTag = 269;
pub const NodeTag_T_AlterSubscriptionStmt: NodeTag = 336;
pub const NodeTag_T_AlterSystemStmt: NodeTag = 328;
pub const NodeTag_T_AlterTSConfigurationStmt: NodeTag = 308;
pub const NodeTag_T_AlterTSDictionaryStmt: NodeTag = 307;
pub const NodeTag_T_AlterTableCmd: NodeTag = 235;
pub const NodeTag_T_AlterTableMoveAllStmt: NodeTag = 317;
pub const NodeTag_T_AlterTableSpaceOptionsStmt: NodeTag = 316;
pub const NodeTag_T_AlterTableStmt: NodeTag = 234;
pub const NodeTag_T_AlterUserMappingStmt: NodeTag = 314;
pub const NodeTag_T_AlternativeSubPlan: NodeTag = 120;
pub const NodeTag_T_AlternativeSubPlanState: NodeTag = 157;
pub const NodeTag_T_Append: NodeTag = 13;
pub const NodeTag_T_AppendPath: NodeTag = 177;
pub const NodeTag_T_AppendRelInfo: NodeTag = 206;
pub const NodeTag_T_AppendState: NodeTag = 62;
pub const NodeTag_T_ArrayCoerceExpr: NodeTag = 125;
pub const NodeTag_T_ArrayExpr: NodeTag = 131;
pub const NodeTag_T_BaseBackupCmd: NodeTag = 394;
pub const NodeTag_T_BitString: NodeTag = 221;
pub const NodeTag_T_BitmapAnd: NodeTag = 16;
pub const NodeTag_T_BitmapAndPath: NodeTag = 168;
pub const NodeTag_T_BitmapAndState: NodeTag = 65;
pub const NodeTag_T_BitmapHeapPath: NodeTag = 167;
pub const NodeTag_T_BitmapHeapScan: NodeTag = 24;
pub const NodeTag_T_BitmapHeapScanState: NodeTag = 73;
pub const NodeTag_T_BitmapIndexScan: NodeTag = 23;
pub const NodeTag_T_BitmapIndexScanState: NodeTag = 72;
pub const NodeTag_T_BitmapOr: NodeTag = 17;
pub const NodeTag_T_BitmapOrPath: NodeTag = 169;
pub const NodeTag_T_BitmapOrState: NodeTag = 66;
pub const NodeTag_T_BoolExpr: NodeTag = 117;
pub const NodeTag_T_BooleanTest: NodeTag = 139;
pub const NodeTag_T_CallContext: NodeTag = 411;
pub const NodeTag_T_CallStmt: NodeTag = 340;
pub const NodeTag_T_CaseExpr: NodeTag = 128;
pub const NodeTag_T_CaseTestExpr: NodeTag = 130;
pub const NodeTag_T_CaseWhen: NodeTag = 129;
pub const NodeTag_T_CheckPointStmt: NodeTag = 281;
pub const NodeTag_T_ClosePortalStmt: NodeTag = 241;
pub const NodeTag_T_ClusterStmt: NodeTag = 242;
pub const NodeTag_T_CoalesceExpr: NodeTag = 134;
pub const NodeTag_T_CoerceToDomain: NodeTag = 140;
pub const NodeTag_T_CoerceToDomainValue: NodeTag = 141;
pub const NodeTag_T_CoerceViaIO: NodeTag = 124;
pub const NodeTag_T_CollateClause: NodeTag = 353;
pub const NodeTag_T_CollateExpr: NodeTag = 127;
pub const NodeTag_T_ColumnDef: NodeTag = 362;
pub const NodeTag_T_ColumnRef: NodeTag = 342;
pub const NodeTag_T_CommentStmt: NodeTag = 248;
pub const NodeTag_T_CommonTableExpr: NodeTag = 384;
pub const NodeTag_T_CompositeTypeStmt: NodeTag = 303;
pub const NodeTag_T_Const: NodeTag = 105;
pub const NodeTag_T_Constraint: NodeTag = 364;
pub const NodeTag_T_ConstraintsSetStmt: NodeTag = 279;
pub const NodeTag_T_ConvertRowtypeExpr: NodeTag = 126;
pub const NodeTag_T_CopyStmt: NodeTag = 243;
pub const NodeTag_T_CreateAmStmt: NodeTag = 332;
pub const NodeTag_T_CreateCastStmt: NodeTag = 287;
pub const NodeTag_T_CreateConversionStmt: NodeTag = 286;
pub const NodeTag_T_CreateDomainStmt: NodeTag = 262;
pub const NodeTag_T_CreateEnumStmt: NodeTag = 304;
pub const NodeTag_T_CreateEventTrigStmt: NodeTag = 324;
pub const NodeTag_T_CreateExtensionStmt: NodeTag = 321;
pub const NodeTag_T_CreateFdwStmt: NodeTag = 309;
pub const NodeTag_T_CreateForeignServerStmt: NodeTag = 311;
pub const NodeTag_T_CreateForeignTableStmt: NodeTag = 319;
pub const NodeTag_T_CreateFunctionStmt: NodeTag = 251;
pub const NodeTag_T_CreateOpClassItem: NodeTag = 375;
pub const NodeTag_T_CreateOpClassStmt: NodeTag = 288;
pub const NodeTag_T_CreateOpFamilyStmt: NodeTag = 289;
pub const NodeTag_T_CreatePLangStmt: NodeTag = 274;
pub const NodeTag_T_CreatePolicyStmt: NodeTag = 329;
pub const NodeTag_T_CreatePublicationStmt: NodeTag = 333;
pub const NodeTag_T_CreateRangeStmt: NodeTag = 305;
pub const NodeTag_T_CreateReplicationSlotCmd: NodeTag = 395;
pub const NodeTag_T_CreateRoleStmt: NodeTag = 275;
pub const NodeTag_T_CreateSchemaStmt: NodeTag = 282;
pub const NodeTag_T_CreateSeqStmt: NodeTag = 268;
pub const NodeTag_T_CreateStatsStmt: NodeTag = 338;
pub const NodeTag_T_CreateStmt: NodeTag = 244;
pub const NodeTag_T_CreateSubscriptionStmt: NodeTag = 335;
pub const NodeTag_T_CreateTableAsStmt: NodeTag = 267;
pub const NodeTag_T_CreateTableSpaceStmt: NodeTag = 295;
pub const NodeTag_T_CreateTransformStmt: NodeTag = 331;
pub const NodeTag_T_CreateTrigStmt: NodeTag = 273;
pub const NodeTag_T_CreateUserMappingStmt: NodeTag = 313;
pub const NodeTag_T_CreatedbStmt: NodeTag = 263;
pub const NodeTag_T_CteScan: NodeTag = 30;
pub const NodeTag_T_CteScanState: NodeTag = 79;
pub const NodeTag_T_CurrentOfExpr: NodeTag = 143;
pub const NodeTag_T_CustomPath: NodeTag = 173;
pub const NodeTag_T_CustomScan: NodeTag = 34;
pub const NodeTag_T_CustomScanState: NodeTag = 83;
pub const NodeTag_T_DeallocateStmt: NodeTag = 293;
pub const NodeTag_T_DeclareCursorStmt: NodeTag = 294;
pub const NodeTag_T_DefElem: NodeTag = 365;
pub const NodeTag_T_DefineStmt: NodeTag = 245;
pub const NodeTag_T_DeleteStmt: NodeTag = 231;
pub const NodeTag_T_DiscardStmt: NodeTag = 272;
pub const NodeTag_T_DistinctExpr: NodeTag = 114;
pub const NodeTag_T_DoStmt: NodeTag = 253;
pub const NodeTag_T_DomainConstraintState: NodeTag = 158;
pub const NodeTag_T_DropOwnedStmt: NodeTag = 301;
pub const NodeTag_T_DropReplicationSlotCmd: NodeTag = 396;
pub const NodeTag_T_DropRoleStmt: NodeTag = 277;
pub const NodeTag_T_DropStmt: NodeTag = 246;
pub const NodeTag_T_DropSubscriptionStmt: NodeTag = 337;
pub const NodeTag_T_DropTableSpaceStmt: NodeTag = 296;
pub const NodeTag_T_DropUserMappingStmt: NodeTag = 315;
pub const NodeTag_T_DropdbStmt: NodeTag = 264;
pub const NodeTag_T_EState: NodeTag = 7;
pub const NodeTag_T_EquivalenceClass: NodeTag = 198;
pub const NodeTag_T_EquivalenceMember: NodeTag = 199;
pub const NodeTag_T_EventTriggerData: NodeTag = 401;
pub const NodeTag_T_ExecuteStmt: NodeTag = 292;
pub const NodeTag_T_ExplainStmt: NodeTag = 266;
pub const NodeTag_T_Expr: NodeTag = 103;
pub const NodeTag_T_ExprState: NodeTag = 152;
pub const NodeTag_T_ExtensibleNode: NodeTag = 226;
pub const NodeTag_T_FdwRoutine: NodeTag = 406;
pub const NodeTag_T_FetchStmt: NodeTag = 249;
pub const NodeTag_T_FieldSelect: NodeTag = 121;
pub const NodeTag_T_FieldStore: NodeTag = 122;
pub const NodeTag_T_Float: NodeTag = 219;
pub const NodeTag_T_ForeignKeyCacheInfo: NodeTag = 410;
pub const NodeTag_T_ForeignKeyOptInfo: NodeTag = 163;
pub const NodeTag_T_ForeignPath: NodeTag = 172;
pub const NodeTag_T_ForeignScan: NodeTag = 33;
pub const NodeTag_T_ForeignScanState: NodeTag = 82;
pub const NodeTag_T_FromExpr: NodeTag = 149;
pub const NodeTag_T_FuncCall: NodeTag = 345;
pub const NodeTag_T_FuncExpr: NodeTag = 111;
pub const NodeTag_T_FunctionParameter: NodeTag = 377;
pub const NodeTag_T_FunctionScan: NodeTag = 27;
pub const NodeTag_T_FunctionScanState: NodeTag = 76;
pub const NodeTag_T_Gather: NodeTag = 45;
pub const NodeTag_T_GatherMerge: NodeTag = 46;
pub const NodeTag_T_GatherMergePath: NodeTag = 183;
pub const NodeTag_T_GatherMergeState: NodeTag = 95;
pub const NodeTag_T_GatherPath: NodeTag = 182;
pub const NodeTag_T_GatherState: NodeTag = 94;
pub const NodeTag_T_GenerationContext: NodeTag = 216;
pub const NodeTag_T_GrantRoleStmt: NodeTag = 239;
pub const NodeTag_T_GrantStmt: NodeTag = 238;
pub const NodeTag_T_Group: NodeTag = 41;
pub const NodeTag_T_GroupPath: NodeTag = 187;
pub const NodeTag_T_GroupResultPath: NodeTag = 179;
pub const NodeTag_T_GroupState: NodeTag = 90;
pub const NodeTag_T_GroupingFunc: NodeTag = 108;
pub const NodeTag_T_GroupingSet: NodeTag = 371;
pub const NodeTag_T_GroupingSetData: NodeTag = 211;
pub const NodeTag_T_GroupingSetsPath: NodeTag = 190;
pub const NodeTag_T_Hash: NodeTag = 47;
pub const NodeTag_T_HashJoin: NodeTag = 38;
pub const NodeTag_T_HashJoinState: NodeTag = 87;
pub const NodeTag_T_HashPath: NodeTag = 176;
pub const NodeTag_T_HashState: NodeTag = 96;
pub const NodeTag_T_IdentifySystemCmd: NodeTag = 393;
pub const NodeTag_T_ImportForeignSchemaStmt: NodeTag = 320;
pub const NodeTag_T_IndexAmRoutine: NodeTag = 407;
pub const NodeTag_T_IndexClause: NodeTag = 203;
pub const NodeTag_T_IndexElem: NodeTag = 363;
pub const NodeTag_T_IndexOnlyScan: NodeTag = 22;
pub const NodeTag_T_IndexOnlyScanState: NodeTag = 71;
pub const NodeTag_T_IndexOptInfo: NodeTag = 162;
pub const NodeTag_T_IndexPath: NodeTag = 166;
pub const NodeTag_T_IndexScan: NodeTag = 21;
pub const NodeTag_T_IndexScanState: NodeTag = 70;
pub const NodeTag_T_IndexStmt: NodeTag = 250;
pub const NodeTag_T_InferClause: NodeTag = 382;
pub const NodeTag_T_InferenceElem: NodeTag = 145;
pub const NodeTag_T_InlineCodeBlock: NodeTag = 405;
pub const NodeTag_T_InsertStmt: NodeTag = 230;
pub const NodeTag_T_IntList: NodeTag = 224;
pub const NodeTag_T_Integer: NodeTag = 218;
pub const NodeTag_T_IntoClause: NodeTag = 151;
pub const NodeTag_T_Join: NodeTag = 35;
pub const NodeTag_T_JoinExpr: NodeTag = 148;
pub const NodeTag_T_JoinState: NodeTag = 84;
pub const NodeTag_T_Limit: NodeTag = 50;
pub const NodeTag_T_LimitPath: NodeTag = 197;
pub const NodeTag_T_LimitState: NodeTag = 99;
pub const NodeTag_T_List: NodeTag = 223;
pub const NodeTag_T_ListenStmt: NodeTag = 257;
pub const NodeTag_T_LoadStmt: NodeTag = 261;
pub const NodeTag_T_LockRows: NodeTag = 49;
pub const NodeTag_T_LockRowsPath: NodeTag = 195;
pub const NodeTag_T_LockRowsState: NodeTag = 98;
pub const NodeTag_T_LockStmt: NodeTag = 278;
pub const NodeTag_T_LockingClause: NodeTag = 378;
pub const NodeTag_T_Material: NodeTag = 39;
pub const NodeTag_T_MaterialPath: NodeTag = 180;
pub const NodeTag_T_MaterialState: NodeTag = 88;
pub const NodeTag_T_MemoryContext: NodeTag = 213;
pub const NodeTag_T_MergeAppend: NodeTag = 14;
pub const NodeTag_T_MergeAppendPath: NodeTag = 178;
pub const NodeTag_T_MergeAppendState: NodeTag = 63;
pub const NodeTag_T_MergeJoin: NodeTag = 37;
pub const NodeTag_T_MergeJoinState: NodeTag = 86;
pub const NodeTag_T_MergePath: NodeTag = 175;
pub const NodeTag_T_MinMaxAggInfo: NodeTag = 208;
pub const NodeTag_T_MinMaxAggPath: NodeTag = 191;
pub const NodeTag_T_MinMaxExpr: NodeTag = 135;
pub const NodeTag_T_ModifyTable: NodeTag = 12;
pub const NodeTag_T_ModifyTablePath: NodeTag = 196;
pub const NodeTag_T_ModifyTableState: NodeTag = 61;
pub const NodeTag_T_MultiAssignRef: NodeTag = 351;
pub const NodeTag_T_NamedArgExpr: NodeTag = 112;
pub const NodeTag_T_NamedTuplestoreScan: NodeTag = 31;
pub const NodeTag_T_NamedTuplestoreScanState: NodeTag = 80;
pub const NodeTag_T_NestLoop: NodeTag = 36;
pub const NodeTag_T_NestLoopParam: NodeTag = 51;
pub const NodeTag_T_NestLoopState: NodeTag = 85;
pub const NodeTag_T_NestPath: NodeTag = 174;
pub const NodeTag_T_NextValueExpr: NodeTag = 144;
pub const NodeTag_T_NotifyStmt: NodeTag = 256;
pub const NodeTag_T_Null: NodeTag = 222;
pub const NodeTag_T_NullIfExpr: NodeTag = 115;
pub const NodeTag_T_NullTest: NodeTag = 138;
pub const NodeTag_T_ObjectWithArgs: NodeTag = 373;
pub const NodeTag_T_OidList: NodeTag = 225;
pub const NodeTag_T_OnConflictClause: NodeTag = 383;
pub const NodeTag_T_OnConflictExpr: NodeTag = 150;
pub const NodeTag_T_OnConflictSetState: NodeTag = 5;
pub const NodeTag_T_OpExpr: NodeTag = 113;
pub const NodeTag_T_Param: NodeTag = 106;
pub const NodeTag_T_ParamPathInfo: NodeTag = 164;
pub const NodeTag_T_ParamRef: NodeTag = 343;
pub const NodeTag_T_PartitionBoundSpec: NodeTag = 389;
pub const NodeTag_T_PartitionCmd: NodeTag = 391;
pub const NodeTag_T_PartitionElem: NodeTag = 387;
pub const NodeTag_T_PartitionPruneInfo: NodeTag = 53;
pub const NodeTag_T_PartitionPruneStepCombine: NodeTag = 56;
pub const NodeTag_T_PartitionPruneStepOp: NodeTag = 55;
pub const NodeTag_T_PartitionRangeDatum: NodeTag = 390;
pub const NodeTag_T_PartitionSpec: NodeTag = 388;
pub const NodeTag_T_PartitionedRelPruneInfo: NodeTag = 54;
pub const NodeTag_T_Path: NodeTag = 165;
pub const NodeTag_T_PathKey: NodeTag = 200;
pub const NodeTag_T_PathTarget: NodeTag = 201;
pub const NodeTag_T_PlaceHolderInfo: NodeTag = 207;
pub const NodeTag_T_PlaceHolderVar: NodeTag = 204;
pub const NodeTag_T_Plan: NodeTag = 9;
pub const NodeTag_T_PlanInvalItem: NodeTag = 57;
pub const NodeTag_T_PlanRowMark: NodeTag = 52;
pub const NodeTag_T_PlanState: NodeTag = 58;
pub const NodeTag_T_PlannedStmt: NodeTag = 229;
pub const NodeTag_T_PlannerGlobal: NodeTag = 160;
pub const NodeTag_T_PlannerInfo: NodeTag = 159;
pub const NodeTag_T_PlannerParamItem: NodeTag = 209;
pub const NodeTag_T_PrepareStmt: NodeTag = 291;
pub const NodeTag_T_ProjectSet: NodeTag = 11;
pub const NodeTag_T_ProjectSetPath: NodeTag = 185;
pub const NodeTag_T_ProjectSetState: NodeTag = 60;
pub const NodeTag_T_ProjectionPath: NodeTag = 184;
pub const NodeTag_T_Query: NodeTag = 228;
pub const NodeTag_T_RangeFunction: NodeTag = 357;
pub const NodeTag_T_RangeSubselect: NodeTag = 356;
pub const NodeTag_T_RangeTableFunc: NodeTag = 359;
pub const NodeTag_T_RangeTableFuncCol: NodeTag = 360;
pub const NodeTag_T_RangeTableSample: NodeTag = 358;
pub const NodeTag_T_RangeTblEntry: NodeTag = 366;
pub const NodeTag_T_RangeTblFunction: NodeTag = 367;
pub const NodeTag_T_RangeTblRef: NodeTag = 147;
pub const NodeTag_T_RangeVar: NodeTag = 101;
pub const NodeTag_T_RawStmt: NodeTag = 227;
pub const NodeTag_T_ReassignOwnedStmt: NodeTag = 302;
pub const NodeTag_T_RecursiveUnion: NodeTag = 15;
pub const NodeTag_T_RecursiveUnionPath: NodeTag = 194;
pub const NodeTag_T_RecursiveUnionState: NodeTag = 64;
pub const NodeTag_T_RefreshMatViewStmt: NodeTag = 326;
pub const NodeTag_T_ReindexStmt: NodeTag = 280;
pub const NodeTag_T_RelOptInfo: NodeTag = 161;
pub const NodeTag_T_RelabelType: NodeTag = 123;
pub const NodeTag_T_RenameStmt: NodeTag = 254;
pub const NodeTag_T_ReplicaIdentityStmt: NodeTag = 327;
pub const NodeTag_T_ResTarget: NodeTag = 350;
pub const NodeTag_T_RestrictInfo: NodeTag = 202;
pub const NodeTag_T_Result: NodeTag = 10;
pub const NodeTag_T_ResultRelInfo: NodeTag = 6;
pub const NodeTag_T_ResultState: NodeTag = 59;
pub const NodeTag_T_ReturnSetInfo: NodeTag = 402;
pub const NodeTag_T_RoleSpec: NodeTag = 385;
pub const NodeTag_T_RollupData: NodeTag = 210;
pub const NodeTag_T_RowCompareExpr: NodeTag = 133;
pub const NodeTag_T_RowExpr: NodeTag = 132;
pub const NodeTag_T_RowMarkClause: NodeTag = 379;
pub const NodeTag_T_RuleStmt: NodeTag = 255;
pub const NodeTag_T_SQLCmd: NodeTag = 399;
pub const NodeTag_T_SQLValueFunction: NodeTag = 136;
pub const NodeTag_T_SampleScan: NodeTag = 20;
pub const NodeTag_T_SampleScanState: NodeTag = 69;
pub const NodeTag_T_ScalarArrayOpExpr: NodeTag = 116;
pub const NodeTag_T_Scan: NodeTag = 18;
pub const NodeTag_T_ScanState: NodeTag = 67;
pub const NodeTag_T_SecLabelStmt: NodeTag = 318;
pub const NodeTag_T_SelectStmt: NodeTag = 233;
pub const NodeTag_T_SeqScan: NodeTag = 19;
pub const NodeTag_T_SeqScanState: NodeTag = 68;
pub const NodeTag_T_SetExprState: NodeTag = 155;
pub const NodeTag_T_SetOp: NodeTag = 48;
pub const NodeTag_T_SetOpPath: NodeTag = 193;
pub const NodeTag_T_SetOpState: NodeTag = 97;
pub const NodeTag_T_SetOperationStmt: NodeTag = 237;
pub const NodeTag_T_SetToDefault: NodeTag = 142;
pub const NodeTag_T_SlabContext: NodeTag = 215;
pub const NodeTag_T_Sort: NodeTag = 40;
pub const NodeTag_T_SortBy: NodeTag = 354;
pub const NodeTag_T_SortGroupClause: NodeTag = 370;
pub const NodeTag_T_SortPath: NodeTag = 186;
pub const NodeTag_T_SortState: NodeTag = 89;
pub const NodeTag_T_SpecialJoinInfo: NodeTag = 205;
pub const NodeTag_T_StartReplicationCmd: NodeTag = 397;
pub const NodeTag_T_StatisticExtInfo: NodeTag = 212;
pub const NodeTag_T_String: NodeTag = 220;
pub const NodeTag_T_SubLink: NodeTag = 118;
pub const NodeTag_T_SubPlan: NodeTag = 119;
pub const NodeTag_T_SubPlanState: NodeTag = 156;
pub const NodeTag_T_SubqueryScan: NodeTag = 26;
pub const NodeTag_T_SubqueryScanPath: NodeTag = 171;
pub const NodeTag_T_SubqueryScanState: NodeTag = 75;
pub const NodeTag_T_SubscriptingRef: NodeTag = 110;
pub const NodeTag_T_SupportRequestCost: NodeTag = 414;
pub const NodeTag_T_SupportRequestIndexCondition: NodeTag = 416;
pub const NodeTag_T_SupportRequestRows: NodeTag = 415;
pub const NodeTag_T_SupportRequestSelectivity: NodeTag = 413;
pub const NodeTag_T_SupportRequestSimplify: NodeTag = 412;
pub const NodeTag_T_TIDBitmap: NodeTag = 404;
pub const NodeTag_T_TableAmRoutine: NodeTag = 408;
pub const NodeTag_T_TableFunc: NodeTag = 102;
pub const NodeTag_T_TableFuncScan: NodeTag = 29;
pub const NodeTag_T_TableFuncScanState: NodeTag = 77;
pub const NodeTag_T_TableLikeClause: NodeTag = 376;
pub const NodeTag_T_TableSampleClause: NodeTag = 368;
pub const NodeTag_T_TargetEntry: NodeTag = 146;
pub const NodeTag_T_TidPath: NodeTag = 170;
pub const NodeTag_T_TidScan: NodeTag = 25;
pub const NodeTag_T_TidScanState: NodeTag = 74;
pub const NodeTag_T_TimeLineHistoryCmd: NodeTag = 398;
pub const NodeTag_T_TransactionStmt: NodeTag = 259;
pub const NodeTag_T_TriggerData: NodeTag = 400;
pub const NodeTag_T_TriggerTransition: NodeTag = 386;
pub const NodeTag_T_TruncateStmt: NodeTag = 247;
pub const NodeTag_T_TsmRoutine: NodeTag = 409;
pub const NodeTag_T_TupleTableSlot: NodeTag = 8;
pub const NodeTag_T_TypeCast: NodeTag = 352;
pub const NodeTag_T_TypeName: NodeTag = 361;
pub const NodeTag_T_Unique: NodeTag = 44;
pub const NodeTag_T_UniquePath: NodeTag = 181;
pub const NodeTag_T_UniqueState: NodeTag = 93;
pub const NodeTag_T_UnlistenStmt: NodeTag = 258;
pub const NodeTag_T_UpdateStmt: NodeTag = 232;
pub const NodeTag_T_UpperUniquePath: NodeTag = 188;
pub const NodeTag_T_VacuumRelation: NodeTag = 392;
pub const NodeTag_T_VacuumStmt: NodeTag = 265;
pub const NodeTag_T_Value: NodeTag = 217;
pub const NodeTag_T_ValuesScan: NodeTag = 28;
pub const NodeTag_T_ValuesScanState: NodeTag = 78;
pub const NodeTag_T_Var: NodeTag = 104;
pub const NodeTag_T_VariableSetStmt: NodeTag = 270;
pub const NodeTag_T_VariableShowStmt: NodeTag = 271;
pub const NodeTag_T_ViewStmt: NodeTag = 260;
pub const NodeTag_T_WindowAgg: NodeTag = 43;
pub const NodeTag_T_WindowAggPath: NodeTag = 192;
pub const NodeTag_T_WindowAggState: NodeTag = 92;
pub const NodeTag_T_WindowClause: NodeTag = 372;
pub const NodeTag_T_WindowDef: NodeTag = 355;
pub const NodeTag_T_WindowFunc: NodeTag = 109;
pub const NodeTag_T_WindowFuncExprState: NodeTag = 154;
pub const NodeTag_T_WindowObjectData: NodeTag = 403;
pub const NodeTag_T_WithCheckOption: NodeTag = 369;
pub const NodeTag_T_WithClause: NodeTag = 381;
pub const NodeTag_T_WorkTableScan: NodeTag = 32;
pub const NodeTag_T_WorkTableScanState: NodeTag = 81;
pub const NodeTag_T_XmlExpr: NodeTag = 137;
pub const NodeTag_T_XmlSerialize: NodeTag = 380;
pub const OIDCHARS: u32 = 10;
pub const OIDVECTORARRAYOID: u32 = 1013;
pub const ObjectType_OBJECT_PROCEDURE: ObjectType = 28;
pub const ObjectType_OBJECT_PUBLICATION: ObjectType = 29;
pub const ObjectType_OBJECT_PUBLICATION_REL: ObjectType = 30;
pub const ObjectType_OBJECT_ROLE: ObjectType = 31;
pub const ObjectType_OBJECT_ROUTINE: ObjectType = 32;
pub const ObjectType_OBJECT_RULE: ObjectType = 33;
pub const ObjectType_OBJECT_SCHEMA: ObjectType = 34;
pub const ObjectType_OBJECT_SEQUENCE: ObjectType = 35;
pub const ObjectType_OBJECT_STATISTIC_EXT: ObjectType = 37;
pub const ObjectType_OBJECT_SUBSCRIPTION: ObjectType = 36;
pub const ObjectType_OBJECT_TABCONSTRAINT: ObjectType = 38;
pub const ObjectType_OBJECT_TABLE: ObjectType = 39;
pub const ObjectType_OBJECT_TABLESPACE: ObjectType = 40;
pub const ObjectType_OBJECT_TRANSFORM: ObjectType = 41;
pub const ObjectType_OBJECT_TRIGGER: ObjectType = 42;
pub const ObjectType_OBJECT_TSCONFIGURATION: ObjectType = 43;
pub const ObjectType_OBJECT_TSDICTIONARY: ObjectType = 44;
pub const ObjectType_OBJECT_TSPARSER: ObjectType = 45;
pub const ObjectType_OBJECT_TSTEMPLATE: ObjectType = 46;
pub const ObjectType_OBJECT_TYPE: ObjectType = 47;
pub const ObjectType_OBJECT_USER_MAPPING: ObjectType = 48;
pub const ObjectType_OBJECT_VIEW: ObjectType = 49;
pub const PACKAGE_BUGREPORT: &'static [u8; 32usize] = b"pgsql-bugs@lists.postgresql.org\0";
pub const PACKAGE_STRING: &'static [u8; 16usize] = b"PostgreSQL 12.3\0";
pub const PACKAGE_VERSION: &'static [u8; 5usize] = b"12.3\0";
pub const PARTITION_STRATEGY_HASH: u8 = 104u8;
pub const PATHARRAYOID: u32 = 1019;
pub const PERFORM_DELETION_CONCURRENT_LOCK: u32 = 32;
pub const PGMCVLISTOID: u32 = 5017;
pub const PGSTAT_NUM_PROGRESS_PARAM: u32 = 20;
pub const PG_BACKEND_VERSIONSTR: &'static [u8; 28usize] = b"postgres (PostgreSQL) 12.3\n\0";
pub const PG_LSNARRAYOID: u32 = 3221;
pub const PG_MAJORVERSION: &'static [u8; 3usize] = b"12\0";
pub const PG_STRERROR_R_BUFLEN: u32 = 256;
pub const PG_VERSION: &'static [u8; 5usize] = b"12.3\0";
pub const PG_VERSION_NUM: u32 = 120003;
pub const PG_VERSION_STR : & 'static [ u8 ; 114usize ] = b"PostgreSQL 12.3 on x86_64-apple-darwin19.0.0, compiled by Apple clang version 11.0.0 (clang-1100.0.33.12), 64-bit\0" ;
pub const POINTARRAYOID: u32 = 1017;
pub const POLYGONARRAYOID: u32 = 1027;
pub const PREDICATELOCK_MANAGER_LWLOCK_OFFSET: u32 = 189;
pub const PROMOTE_SIGNAL_FILE: &'static [u8; 8usize] = b"promote\0";
pub const PVC_INCLUDE_AGGREGATES: u32 = 1;
pub const PVC_INCLUDE_PLACEHOLDERS: u32 = 16;
pub const PVC_INCLUDE_WINDOWFUNCS: u32 = 4;
pub const PVC_RECURSE_AGGREGATES: u32 = 2;
pub const PVC_RECURSE_PLACEHOLDERS: u32 = 32;
pub const PVC_RECURSE_WINDOWFUNCS: u32 = 8;
pub const ParseExprKind_EXPR_KIND_ALTER_COL_TRANSFORM: ParseExprKind = 32;
pub const ParseExprKind_EXPR_KIND_CALL_ARGUMENT: ParseExprKind = 38;
pub const ParseExprKind_EXPR_KIND_CHECK_CONSTRAINT: ParseExprKind = 26;
pub const ParseExprKind_EXPR_KIND_COLUMN_DEFAULT: ParseExprKind = 28;
pub const ParseExprKind_EXPR_KIND_COPY_WHERE: ParseExprKind = 39;
pub const ParseExprKind_EXPR_KIND_DISTINCT_ON: ParseExprKind = 20;
pub const ParseExprKind_EXPR_KIND_DOMAIN_CHECK: ParseExprKind = 27;
pub const ParseExprKind_EXPR_KIND_EXECUTE_PARAMETER: ParseExprKind = 33;
pub const ParseExprKind_EXPR_KIND_FUNCTION_DEFAULT: ParseExprKind = 29;
pub const ParseExprKind_EXPR_KIND_GENERATED_COLUMN: ParseExprKind = 40;
pub const ParseExprKind_EXPR_KIND_GROUP_BY: ParseExprKind = 18;
pub const ParseExprKind_EXPR_KIND_INDEX_EXPRESSION: ParseExprKind = 30;
pub const ParseExprKind_EXPR_KIND_INDEX_PREDICATE: ParseExprKind = 31;
pub const ParseExprKind_EXPR_KIND_INSERT_TARGET: ParseExprKind = 15;
pub const ParseExprKind_EXPR_KIND_LIMIT: ParseExprKind = 21;
pub const ParseExprKind_EXPR_KIND_OFFSET: ParseExprKind = 22;
pub const ParseExprKind_EXPR_KIND_ORDER_BY: ParseExprKind = 19;
pub const ParseExprKind_EXPR_KIND_PARTITION_BOUND: ParseExprKind = 36;
pub const ParseExprKind_EXPR_KIND_PARTITION_EXPRESSION: ParseExprKind = 37;
pub const ParseExprKind_EXPR_KIND_POLICY: ParseExprKind = 35;
pub const ParseExprKind_EXPR_KIND_RETURNING: ParseExprKind = 23;
pub const ParseExprKind_EXPR_KIND_SELECT_TARGET: ParseExprKind = 14;
pub const ParseExprKind_EXPR_KIND_TRIGGER_WHEN: ParseExprKind = 34;
pub const ParseExprKind_EXPR_KIND_UPDATE_SOURCE: ParseExprKind = 16;
pub const ParseExprKind_EXPR_KIND_UPDATE_TARGET: ParseExprKind = 17;
pub const ParseExprKind_EXPR_KIND_VALUES: ParseExprKind = 24;
pub const ParseExprKind_EXPR_KIND_VALUES_SINGLE: ParseExprKind = 25;
pub const ParseExprKind_EXPR_KIND_WINDOW_FRAME_GROUPS: ParseExprKind = 13;
pub const PartitionPruneCombineOp_PARTPRUNE_COMBINE_INTERSECT: PartitionPruneCombineOp = 1;
pub const PartitionPruneCombineOp_PARTPRUNE_COMBINE_UNION: PartitionPruneCombineOp = 0;
pub const PartitionwiseAggregateType_PARTITIONWISE_AGGREGATE_FULL: PartitionwiseAggregateType = 1;
pub const PartitionwiseAggregateType_PARTITIONWISE_AGGREGATE_NONE: PartitionwiseAggregateType = 0;
pub const PartitionwiseAggregateType_PARTITIONWISE_AGGREGATE_PARTIAL: PartitionwiseAggregateType =
    2;
pub const PlanCacheMode_PLAN_CACHE_MODE_AUTO: PlanCacheMode = 0;
pub const PlanCacheMode_PLAN_CACHE_MODE_FORCE_CUSTOM_PLAN: PlanCacheMode = 2;
pub const PlanCacheMode_PLAN_CACHE_MODE_FORCE_GENERIC_PLAN: PlanCacheMode = 1;
pub const ProcessUtilityContext_PROCESS_UTILITY_QUERY_NONATOMIC: ProcessUtilityContext = 2;
pub const ProcessUtilityContext_PROCESS_UTILITY_SUBCOMMAND: ProcessUtilityContext = 3;
pub const ProgressCommandType_PROGRESS_COMMAND_CLUSTER: ProgressCommandType = 2;
pub const ProgressCommandType_PROGRESS_COMMAND_CREATE_INDEX: ProgressCommandType = 3;
pub const QTW_DONT_COPY_QUERY: u32 = 64;
pub const QTW_EXAMINE_RTES_AFTER: u32 = 32;
pub const QTW_EXAMINE_RTES_BEFORE: u32 = 16;
pub const RECOVERY_SIGNAL_FILE: &'static [u8; 16usize] = b"recovery.signal\0";
pub const REFCURSORARRAYOID: u32 = 2201;
pub const REGCLASSARRAYOID: u32 = 2210;
pub const REGCONFIGARRAYOID: u32 = 3735;
pub const REGDICTIONARYARRAYOID: u32 = 3770;
pub const REGNAMESPACEARRAYOID: u32 = 4090;
pub const REGOPERARRAYOID: u32 = 2208;
pub const REGOPERATORARRAYOID: u32 = 2209;
pub const REGPROCARRAYOID: u32 = 1008;
pub const REGPROCEDUREARRAYOID: u32 = 2207;
pub const REGROLEARRAYOID: u32 = 4097;
pub const REINDEXOPT_REPORT_PROGRESS: u32 = 2;
pub const RELCACHE_INIT_FILENAME: &'static [u8; 17usize] = b"pg_internal.init\0";
pub const RELKIND_PARTITIONED_INDEX: u8 = 73u8;
pub const RTEKind_RTE_RESULT: RTEKind = 8;
pub const RTMaxStrategyNumber: u32 = 28;
pub const RTPrefixStrategyNumber: u32 = 28;
pub const RVROption_RVR_MISSING_OK: RVROption = 1;
pub const RVROption_RVR_NOWAIT: RVROption = 2;
pub const RVROption_RVR_SKIP_LOCKED: RVROption = 4;
pub const RecoveryTargetTimeLineGoal_RECOVERY_TARGET_TIMELINE_CONTROLFILE:
    RecoveryTargetTimeLineGoal = 0;
pub const RecoveryTargetTimeLineGoal_RECOVERY_TARGET_TIMELINE_LATEST: RecoveryTargetTimeLineGoal =
    1;
pub const RecoveryTargetTimeLineGoal_RECOVERY_TARGET_TIMELINE_NUMERIC: RecoveryTargetTimeLineGoal =
    2;
pub const RelOptKind_RELOPT_DEADREL: RelOptKind = 6;
pub const RelOptKind_RELOPT_OTHER_JOINREL: RelOptKind = 3;
pub const RelOptKind_RELOPT_OTHER_UPPER_REL: RelOptKind = 5;
pub const RelOptKind_RELOPT_UPPER_REL: RelOptKind = 4;
pub const SHARED_TUPLESTORE_SINGLE_PASS: u32 = 1;
pub const SIZEOF_BOOL: u32 = 1;
pub const SPI_OPT_NONATOMIC: u32 = 1;
pub const STANDBY_SIGNAL_FILE: &'static [u8; 15usize] = b"standby.signal\0";
pub const ScanOptions_SO_ALLOW_PAGEMODE: ScanOptions = 64;
pub const ScanOptions_SO_ALLOW_STRAT: ScanOptions = 16;
pub const ScanOptions_SO_ALLOW_SYNC: ScanOptions = 32;
pub const ScanOptions_SO_TEMP_SNAPSHOT: ScanOptions = 128;
pub const ScanOptions_SO_TYPE_ANALYZE: ScanOptions = 8;
pub const ScanOptions_SO_TYPE_BITMAPSCAN: ScanOptions = 2;
pub const ScanOptions_SO_TYPE_SAMPLESCAN: ScanOptions = 4;
pub const ScanOptions_SO_TYPE_SEQSCAN: ScanOptions = 1;
pub const ScanOptions_SO_TYPE_TIDSCAN: ScanOptions = 256;
pub const SelfItemPointerAttributeNumber: i32 = -1;
pub const SnapshotType_SNAPSHOT_ANY: SnapshotType = 2;
pub const SnapshotType_SNAPSHOT_DIRTY: SnapshotType = 4;
pub const SnapshotType_SNAPSHOT_HISTORIC_MVCC: SnapshotType = 5;
pub const SnapshotType_SNAPSHOT_MVCC: SnapshotType = 0;
pub const SnapshotType_SNAPSHOT_NON_VACUUMABLE: SnapshotType = 6;
pub const SnapshotType_SNAPSHOT_SELF: SnapshotType = 1;
pub const SnapshotType_SNAPSHOT_TOAST: SnapshotType = 3;
pub const StatMsgType_PGSTAT_MTYPE_CHECKSUMFAILURE: StatMsgType = 18;
pub const SysCacheIdentifier_STATEXTDATASTXOID: SysCacheIdentifier = 55;
pub const SysCacheIdentifier_STATEXTNAMENSP: SysCacheIdentifier = 56;
pub const SysCacheIdentifier_STATEXTOID: SysCacheIdentifier = 57;
pub const SysCacheIdentifier_STATRELATTINH: SysCacheIdentifier = 58;
pub const SysCacheIdentifier_SUBSCRIPTIONNAME: SysCacheIdentifier = 59;
pub const SysCacheIdentifier_SUBSCRIPTIONOID: SysCacheIdentifier = 60;
pub const SysCacheIdentifier_SUBSCRIPTIONRELMAP: SysCacheIdentifier = 61;
pub const SysCacheIdentifier_TABLESPACEOID: SysCacheIdentifier = 62;
pub const SysCacheIdentifier_TRFOID: SysCacheIdentifier = 63;
pub const SysCacheIdentifier_TRFTYPELANG: SysCacheIdentifier = 64;
pub const SysCacheIdentifier_TSCONFIGMAP: SysCacheIdentifier = 65;
pub const SysCacheIdentifier_TSCONFIGNAMENSP: SysCacheIdentifier = 66;
pub const SysCacheIdentifier_TSCONFIGOID: SysCacheIdentifier = 67;
pub const SysCacheIdentifier_TSDICTNAMENSP: SysCacheIdentifier = 68;
pub const SysCacheIdentifier_TSDICTOID: SysCacheIdentifier = 69;
pub const SysCacheIdentifier_TSPARSERNAMENSP: SysCacheIdentifier = 70;
pub const SysCacheIdentifier_TSPARSEROID: SysCacheIdentifier = 71;
pub const SysCacheIdentifier_TSTEMPLATENAMENSP: SysCacheIdentifier = 72;
pub const SysCacheIdentifier_TSTEMPLATEOID: SysCacheIdentifier = 73;
pub const SysCacheIdentifier_TYPENAMENSP: SysCacheIdentifier = 74;
pub const SysCacheIdentifier_TYPEOID: SysCacheIdentifier = 75;
pub const SysCacheIdentifier_USERMAPPINGOID: SysCacheIdentifier = 76;
pub const SysCacheIdentifier_USERMAPPINGUSERSERVER: SysCacheIdentifier = 77;
pub const TABLE_AM_HANDLEROID: u32 = 269;
pub const TABLE_INSERT_FROZEN: u32 = 4;
pub const TABLE_INSERT_NO_LOGICAL: u32 = 8;
pub const TABLE_INSERT_SKIP_FSM: u32 = 2;
pub const TABLE_INSERT_SKIP_WAL: u32 = 1;
pub const TIDARRAYOID: u32 = 1010;
pub const TIMEARRAYOID: u32 = 1183;
pub const TIMESTAMPARRAYOID: u32 = 1115;
pub const TIMESTAMPTZARRAYOID: u32 = 1185;
pub const TIMETZARRAYOID: u32 = 1270;
pub const TM_Result_TM_BeingModified: TM_Result = 5;
pub const TM_Result_TM_Deleted: TM_Result = 4;
pub const TM_Result_TM_Invisible: TM_Result = 1;
pub const TM_Result_TM_Ok: TM_Result = 0;
pub const TM_Result_TM_SelfModified: TM_Result = 2;
pub const TM_Result_TM_Updated: TM_Result = 3;
pub const TM_Result_TM_WouldBlock: TM_Result = 6;
pub const TRANSACTION_STATUS_ABORTED: u32 = 2;
pub const TRANSACTION_STATUS_COMMITTED: u32 = 1;
pub const TRANSACTION_STATUS_IN_PROGRESS: u32 = 0;
pub const TRANSACTION_STATUS_SUB_COMMITTED: u32 = 3;
pub const TSQUERYARRAYOID: u32 = 3645;
pub const TSRANGEARRAYOID: u32 = 3909;
pub const TSRANGEOID: u32 = 3908;
pub const TSTZRANGEARRAYOID: u32 = 3911;
pub const TSTZRANGEOID: u32 = 3910;
pub const TSVECTORARRAYOID: u32 = 3643;
pub const TTS_FLAG_EMPTY: u32 = 2;
pub const TTS_FLAG_FIXED: u32 = 16;
pub const TTS_FLAG_SHOULDFREE: u32 = 4;
pub const TTS_FLAG_SLOW: u32 = 8;
pub const TUPLE_LOCK_FLAG_FIND_LAST_VERSION: u32 = 2;
pub const TUPLE_LOCK_FLAG_LOCK_UPDATE_IN_PROGRESS: u32 = 1;
pub const TXID_SNAPSHOTARRAYOID: u32 = 2949;
pub const TXID_SNAPSHOTOID: u32 = 2970;
pub const TYPECACHE_DOMAIN_BASE_INFO: u32 = 4096;
pub const TYPECACHE_DOMAIN_CONSTR_INFO: u32 = 8192;
pub const TYPECACHE_HASH_EXTENDED_PROC: u32 = 16384;
pub const TYPECACHE_HASH_EXTENDED_PROC_FINFO: u32 = 32768;
pub const TableLikeOption_CREATE_TABLE_LIKE_COMMENTS: TableLikeOption = 1;
pub const TableLikeOption_CREATE_TABLE_LIKE_DEFAULTS: TableLikeOption = 4;
pub const TableLikeOption_CREATE_TABLE_LIKE_GENERATED: TableLikeOption = 8;
pub const TableLikeOption_CREATE_TABLE_LIKE_IDENTITY: TableLikeOption = 16;
pub const TableLikeOption_CREATE_TABLE_LIKE_INDEXES: TableLikeOption = 32;
pub const TableLikeOption_CREATE_TABLE_LIKE_STATISTICS: TableLikeOption = 64;
pub const TableLikeOption_CREATE_TABLE_LIKE_STORAGE: TableLikeOption = 128;
pub const TableOidAttributeNumber: i32 = -6;
pub const TempNamespaceStatus_TEMP_NAMESPACE_IDLE: TempNamespaceStatus = 1;
pub const TempNamespaceStatus_TEMP_NAMESPACE_IN_USE: TempNamespaceStatus = 2;
pub const TempNamespaceStatus_TEMP_NAMESPACE_NOT_TEMP: TempNamespaceStatus = 0;
pub const TuplesortMethod_SORT_TYPE_EXTERNAL_MERGE: TuplesortMethod = 4;
pub const TuplesortMethod_SORT_TYPE_EXTERNAL_SORT: TuplesortMethod = 3;
pub const TuplesortMethod_SORT_TYPE_QUICKSORT: TuplesortMethod = 2;
pub const TuplesortMethod_SORT_TYPE_STILL_IN_PROGRESS: TuplesortMethod = 0;
pub const TuplesortMethod_SORT_TYPE_TOP_N_HEAPSORT: TuplesortMethod = 1;
pub const TuplesortSpaceType_SORT_SPACE_TYPE_DISK: TuplesortSpaceType = 0;
pub const TuplesortSpaceType_SORT_SPACE_TYPE_MEMORY: TuplesortSpaceType = 1;
pub const TypeFuncClass_TYPEFUNC_COMPOSITE_DOMAIN: TypeFuncClass = 2;
pub const TypeFuncClass_TYPEFUNC_OTHER: TypeFuncClass = 4;
pub const TypeFuncClass_TYPEFUNC_RECORD: TypeFuncClass = 3;
pub const USE_REPL_SNPRINTF: u32 = 1;
pub const UUIDARRAYOID: u32 = 2951;
pub const UpperRelationKind_UPPERREL_DISTINCT: UpperRelationKind = 4;
pub const UpperRelationKind_UPPERREL_FINAL: UpperRelationKind = 6;
pub const UpperRelationKind_UPPERREL_GROUP_AGG: UpperRelationKind = 2;
pub const UpperRelationKind_UPPERREL_ORDERED: UpperRelationKind = 5;
pub const UpperRelationKind_UPPERREL_PARTIAL_GROUP_AGG: UpperRelationKind = 1;
pub const UpperRelationKind_UPPERREL_WINDOW: UpperRelationKind = 3;
pub const VARBITARRAYOID: u32 = 1563;
pub const VARCHARARRAYOID: u32 = 1015;
pub const WL_EXIT_ON_PM_DEATH: u32 = 32;
pub const WaitEventActivity_WAIT_EVENT_LOGICAL_APPLY_MAIN: WaitEventActivity = 83886085;
pub const WaitEventActivity_WAIT_EVENT_LOGICAL_LAUNCHER_MAIN: WaitEventActivity = 83886086;
pub const WaitEventClient_WAIT_EVENT_GSS_OPEN_SERVER: WaitEventClient = 100663304;
pub const WaitEventIO_WAIT_EVENT_WAL_SYNC: WaitEventIO = 167772225;
pub const WaitEventIO_WAIT_EVENT_WAL_SYNC_METHOD_ASSIGN: WaitEventIO = 167772226;
pub const WaitEventIO_WAIT_EVENT_WAL_WRITE: WaitEventIO = 167772227;
pub const WaitEventIPC_WAIT_EVENT_CHECKPOINT_DONE: WaitEventIPC = 134217732;
pub const WaitEventIPC_WAIT_EVENT_CHECKPOINT_START: WaitEventIPC = 134217733;
pub const WaitEventIPC_WAIT_EVENT_CLOG_GROUP_UPDATE: WaitEventIPC = 134217731;
pub const WaitEventIPC_WAIT_EVENT_EXECUTE_GATHER: WaitEventIPC = 134217734;
pub const WaitEventIPC_WAIT_EVENT_HASH_BATCH_ALLOCATING: WaitEventIPC = 134217735;
pub const WaitEventIPC_WAIT_EVENT_HASH_BATCH_ELECTING: WaitEventIPC = 134217736;
pub const WaitEventIPC_WAIT_EVENT_HASH_BATCH_LOADING: WaitEventIPC = 134217737;
pub const WaitEventIPC_WAIT_EVENT_HASH_BUILD_ALLOCATING: WaitEventIPC = 134217738;
pub const WaitEventIPC_WAIT_EVENT_HASH_BUILD_ELECTING: WaitEventIPC = 134217739;
pub const WaitEventIPC_WAIT_EVENT_HASH_BUILD_HASHING_INNER: WaitEventIPC = 134217740;
pub const WaitEventIPC_WAIT_EVENT_HASH_BUILD_HASHING_OUTER: WaitEventIPC = 134217741;
pub const WaitEventIPC_WAIT_EVENT_HASH_GROW_BATCHES_ALLOCATING: WaitEventIPC = 134217742;
pub const WaitEventIPC_WAIT_EVENT_HASH_GROW_BATCHES_DECIDING: WaitEventIPC = 134217743;
pub const WaitEventIPC_WAIT_EVENT_HASH_GROW_BATCHES_ELECTING: WaitEventIPC = 134217744;
pub const WaitEventIPC_WAIT_EVENT_HASH_GROW_BATCHES_FINISHING: WaitEventIPC = 134217745;
pub const WaitEventIPC_WAIT_EVENT_HASH_GROW_BATCHES_REPARTITIONING: WaitEventIPC = 134217746;
pub const WaitEventIPC_WAIT_EVENT_HASH_GROW_BUCKETS_ALLOCATING: WaitEventIPC = 134217747;
pub const WaitEventIPC_WAIT_EVENT_HASH_GROW_BUCKETS_ELECTING: WaitEventIPC = 134217748;
pub const WaitEventIPC_WAIT_EVENT_HASH_GROW_BUCKETS_REINSERTING: WaitEventIPC = 134217749;
pub const WaitEventIPC_WAIT_EVENT_LOGICAL_SYNC_DATA: WaitEventIPC = 134217750;
pub const WaitEventIPC_WAIT_EVENT_LOGICAL_SYNC_STATE_CHANGE: WaitEventIPC = 134217751;
pub const WaitEventIPC_WAIT_EVENT_MQ_INTERNAL: WaitEventIPC = 134217752;
pub const WaitEventIPC_WAIT_EVENT_MQ_PUT_MESSAGE: WaitEventIPC = 134217753;
pub const WaitEventIPC_WAIT_EVENT_MQ_RECEIVE: WaitEventIPC = 134217754;
pub const WaitEventIPC_WAIT_EVENT_MQ_SEND: WaitEventIPC = 134217755;
pub const WaitEventIPC_WAIT_EVENT_PARALLEL_BITMAP_SCAN: WaitEventIPC = 134217756;
pub const WaitEventIPC_WAIT_EVENT_PARALLEL_CREATE_INDEX_SCAN: WaitEventIPC = 134217757;
pub const WaitEventIPC_WAIT_EVENT_PARALLEL_FINISH: WaitEventIPC = 134217758;
pub const WaitEventIPC_WAIT_EVENT_PROCARRAY_GROUP_UPDATE: WaitEventIPC = 134217759;
pub const WaitEventIPC_WAIT_EVENT_PROMOTE: WaitEventIPC = 134217760;
pub const WaitEventIPC_WAIT_EVENT_REPLICATION_ORIGIN_DROP: WaitEventIPC = 134217761;
pub const WaitEventIPC_WAIT_EVENT_REPLICATION_SLOT_DROP: WaitEventIPC = 134217762;
pub const WaitEventIPC_WAIT_EVENT_SAFE_SNAPSHOT: WaitEventIPC = 134217763;
pub const WaitEventIPC_WAIT_EVENT_SYNC_REP: WaitEventIPC = 134217764;
pub const XACT_FLAGS_ACCESSEDTEMPNAMESPACE: u32 = 1;
pub const XACT_XINFO_HAS_GID: u32 = 128;
pub const XIDARRAYOID: u32 = 1011;
pub const XMLARRAYOID: u32 = 143;
pub const dsm_op_DSM_OP_DESTROY: dsm_op = 3;
pub const tuplehash_status_tuplehash_SH_EMPTY: tuplehash_status = 0;
pub const tuplehash_status_tuplehash_SH_IN_USE: tuplehash_status = 1;
pub type BeginForeignInsert_function = ::std::option::Option<
    unsafe extern "C" fn(mtstate: *mut ModifyTableState, rinfo: *mut ResultRelInfo),
>;
pub type CTEMaterialize = u32;
pub type ClusterOption = u32;
pub type EndForeignInsert_function =
    ::std::option::Option<unsafe extern "C" fn(estate: *mut EState, rinfo: *mut ResultRelInfo)>;
pub type ExplainOneQuery_hook_type = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Query,
        cursorOptions: ::std::os::raw::c_int,
        into: *mut IntoClause,
        es: *mut ExplainState,
        queryString: *const ::std::os::raw::c_char,
        params: ParamListInfo,
        queryEnv: *mut QueryEnvironment,
    ),
>;
pub type ForceParallelMode = u32;
pub type FunctionCallInfo = *mut FunctionCallInfoBaseData;
pub type GetForeignUpperPaths_function = ::std::option::Option<
    unsafe extern "C" fn(
        root: *mut PlannerInfo,
        stage: UpperRelationKind,
        input_rel: *mut RelOptInfo,
        output_rel: *mut RelOptInfo,
        extra: *mut ::std::os::raw::c_void,
    ),
>;
pub type InheritanceKind = u32;
pub type MemoryStatsPrintFunc = ::std::option::Option<
    unsafe extern "C" fn(
        context: MemoryContext,
        passthru: *mut ::std::os::raw::c_void,
        stats_string: *const ::std::os::raw::c_char,
    ),
>;
pub type ParallelBlockTableScanDesc = *mut ParallelBlockTableScanDescData;
pub type ParallelTableScanDesc = *mut ParallelTableScanDescData;
pub type ParamCompileHook = ::std::option::Option<
    unsafe extern "C" fn(
        params: ParamListInfo,
        param: *mut Param,
        state: *mut ExprState,
        resv: *mut Datum,
        resnull: *mut bool,
    ),
>;
pub type ParamFetchHook = ::std::option::Option<
    unsafe extern "C" fn(
        params: ParamListInfo,
        paramid: ::std::os::raw::c_int,
        speculative: bool,
        workspace: *mut ParamExternData,
    ) -> *mut ParamExternData,
>;
pub type PartitionDirectory = *mut PartitionDirectoryData;
pub type PartitionPruneCombineOp = u32;
pub type PartitionScheme = *mut PartitionSchemeData;
pub type PartitionwiseAggregateType = u32;
pub type PlanCacheMode = u32;
pub type RVROption = u32;
pub type RecoveryTargetTimeLineGoal = u32;
pub type RefetchForeignRow_function = ::std::option::Option<
    unsafe extern "C" fn(
        estate: *mut EState,
        erm: *mut ExecRowMark,
        rowid: Datum,
        slot: *mut TupleTableSlot,
        updated: *mut bool,
    ),
>;
pub type ReparameterizeForeignPathByChild_function = ::std::option::Option<
    unsafe extern "C" fn(
        root: *mut PlannerInfo,
        fdw_private: *mut List,
        child_rel: *mut RelOptInfo,
    ) -> *mut List,
>;
pub type ScanOptions = u32;
pub type SnapshotType = u32;
pub type SortCoordinate = *mut SortCoordinateData;
pub type TM_Result = u32;
pub type TableScanDesc = *mut TableScanDescData;
pub type TempNamespaceStatus = u32;
pub type TupleDesc = *mut TupleDescData;
pub type TuplesortMethod = u32;
pub type TuplesortSpaceType = u32;
pub type XidStatus = ::std::os::raw::c_int;
pub type ambuildphasename_function =
    ::std::option::Option<unsafe extern "C" fn(phasenum: int64) -> *mut ::std::os::raw::c_char>;
pub type bitmapword = uint64;
pub type create_upper_paths_hook_type = ::std::option::Option<
    unsafe extern "C" fn(
        root: *mut PlannerInfo,
        stage: UpperRelationKind,
        input_rel: *mut RelOptInfo,
        output_rel: *mut RelOptInfo,
        extra: *mut ::std::os::raw::c_void,
    ),
>;
pub type list_qsort_comparator = ::std::option::Option<
    unsafe extern "C" fn(
        a: *const ::std::os::raw::c_void,
        b: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type signedbitmapword = int64;
pub type validate_string_relopt =
    ::std::option::Option<unsafe extern "C" fn(value: *const ::std::os::raw::c_char)>;
pub type xl_xact_parsed_prepare = xl_xact_parsed_commit;
